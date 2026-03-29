use crate::core::{DeriveInput, FactoryExpr, error};
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Expr, PatType, Token, Type,
    parse::{Parse, ParseStream},
    spanned::Spanned,
};

enum InjectExpr {
    /// A direct expression, optionally with factory inputs: `expr` or `|dep: T| expr`
    Value(Box<Expr>, Vec<PatType>),
    /// A named injection tag type: `named(TagType)`
    Named(Type),
    /// A named injection string key: `named("key")` — resolved via `Key<{hash}>`
    NamedStr(u128),
}
impl Parse for InjectExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Check for `named(TagType)` or `named("string_key")` syntax
        if input.peek(syn::Ident) {
            let fork = input.fork();
            if let Ok(ident) = fork.parse::<syn::Ident>() {
                if ident == "named" {
                    // Commit to the `named(...)` parse
                    input.parse::<syn::Ident>()?; // consume "named"
                    let content;
                    syn::parenthesized!(content in input);
                    // Check for string literal: named("key")
                    if content.peek(syn::LitStr) {
                        let lit: syn::LitStr = content.parse()?;
                        let hash_bytes = crate::core::hash::fnv(lit.value().as_bytes());
                        let hash = u128::from_be_bytes(hash_bytes);
                        return Ok(InjectExpr::NamedStr(hash));
                    }
                    // Otherwise parse as type: named(TagType)
                    let tag_type: Type = content.parse()?;
                    return Ok(InjectExpr::Named(tag_type));
                }
            }
        }
        if input.peek(Token![|]) {
            let expr = FactoryExpr::parse(input)?;
            Ok(InjectExpr::Value(expr.body, expr.inputs))
        } else {
            Ok(InjectExpr::Value(input.parse()?, vec![]))
        }
    }
}

pub(crate) fn handle_injectable(item: TokenStream) -> syn::Result<TokenStream> {
    let input = syn::parse::<DeriveInput>(item)?;
    let ident = &input.ident;
    let fields = input.fields();
    let types = input.field_types();
    let keys = input.field_idents();
    let attributes = fields
        .iter()
        .map(|f| {
            let Some(attr) = f.attrs.iter().rfind(|a| a.path().is_ident("inject")) else {
                return Ok(None);
            };
            attr.parse_args::<InjectExpr>().map(Some).map_err(|e| {
                error::combine(
                    syn::Error::new(attr.span(), "Unable to parse inject attribute"),
                    e,
                )
            })
        })
        .collect::<syn::Result<Vec<_>>>()?;
    let generic_params = input.generic_params();
    let generic_keys = input.generic_keys();
    let lifetime_keys = input.lifetime_keys();
    let prov_lifetimes = match lifetime_keys.is_empty() {
        false => quote! { 'prov: #(#lifetime_keys)+*, },
        true => quote! {},
    };
    let where_predicates = match &input.generics.where_clause {
        Some(w) => {
            let predicates = &w.predicates;
            quote! { #predicates }
        }
        None => quote! {},
    };
    let creation_output = match keys.is_empty() && !types.is_empty() {
        true => {
            let items = types.iter().zip(&attributes).map(|(ty, a)| match a {
                Some(InjectExpr::Named(tag)) => {
                    quote! { nject::Named::<#tag, #ty>::into_inner(provider.provide()) }
                }
                Some(InjectExpr::NamedStr(hash)) => {
                    quote! { nject::Named::<nject::Key<#hash>, #ty>::into_inner(provider.provide()) }
                }
                Some(InjectExpr::Value(output, inputs)) => {
                    let input_stmts = inputs
                        .iter()
                        .map(|x| quote! { let #x = provider.provide(); })
                        .collect::<Vec<_>>();
                    if input_stmts.is_empty() {
                        quote! { #output }
                    } else {
                        quote! {
                            {
                                #(#input_stmts)*
                                #output
                            }
                        }
                    }
                }
                None => quote! { provider.provide() },
            });
            quote! { #ident(#(#items),*) }
        }
        false => {
            let items = keys.iter().zip(types.iter()).zip(&attributes).map(|((k, ty), a)| match a {
                Some(InjectExpr::Named(tag)) => {
                    quote! {
                        #k: nject::Named::<#tag, #ty>::into_inner(provider.provide())
                    }
                }
                Some(InjectExpr::NamedStr(hash)) => {
                    quote! {
                        #k: nject::Named::<nject::Key<#hash>, #ty>::into_inner(provider.provide())
                    }
                }
                Some(InjectExpr::Value(output, inputs)) => {
                    let input_stmts = inputs
                        .iter()
                        .map(|x| quote! { let #x = provider.provide(); })
                        .collect::<Vec<_>>();
                    quote! {
                        #k: {
                            #(#input_stmts)*
                            #output
                        }
                    }
                }
                None => quote! { #k: provider.provide() },
            });
            quote! { #ident { #(#items),* } }
        }
    };
    let mut prov_types = Vec::<_>::with_capacity(types.len());
    for (t, a) in types.iter().zip(&attributes) {
        match a {
            Some(InjectExpr::Named(tag)) => {
                prov_types.push(quote! { nject::Named<#tag, #t> });
            }
            Some(InjectExpr::NamedStr(hash)) => {
                prov_types.push(quote! { nject::Named<nject::Key<#hash>, #t> });
            }
            Some(InjectExpr::Value(_, inputs)) => {
                for attr_type in inputs.iter().map(|x| &x.ty) {
                    prov_types.push(quote! {#attr_type});
                }
            }
            None => {
                prov_types.push(quote! {#t});
            }
        }
    }
    prov_types.dedup_by(|a, b| a.to_string() == b.to_string());
    let output = quote! {
        #[derive(nject::InjectableHelperAttr)]
        #input

        impl<'prov, #(#generic_params,)*NjectProvider> nject::Injectable<'prov, #ident<#(#generic_keys),*>, NjectProvider> for #ident<#(#generic_keys),*>
            where
                #prov_lifetimes
                NjectProvider: #(nject::Provider<'prov, #prov_types>)+*, #where_predicates
        {
            #[inline]
            fn inject(provider: &'prov NjectProvider) -> #ident<#(#generic_keys),*> {
                #creation_output
            }
        }
    };
    Ok(output.into())
}
