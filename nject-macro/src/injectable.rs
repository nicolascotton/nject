use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    DeriveInput, Expr, GenericParam, Ident, Token, Type, TypeParam,
};

struct InjectHelperExpr(Expr, Punctuated<TypeParam, Token![,]>);
impl Parse for InjectHelperExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let parsed_type = input.parse()?;
        if input.parse::<Token![,]>().is_ok() {
            let parsed_value = Punctuated::parse_separated_nonempty(&input)?;
            Ok(Self(parsed_type, parsed_value))
        } else {
            Ok(Self(parsed_type, Punctuated::new()))
        }
    }
}

pub(crate) fn handle_injectable(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let ident = &input.ident;
    let fields = match &input.data {
        syn::Data::Struct(d) => &d.fields,
        _ => panic!("Unsupported type. Macro should be used on a struct"),
    };
    let types = fields.iter().map(|f| &f.ty).collect::<Vec<&Type>>();
    let keys = fields
        .iter()
        .map(|f| f.ident.as_ref())
        .filter_map(|i| i)
        .collect::<Vec<&Ident>>();
    let attributes = fields
        .iter()
        .map(|f| {
            match f
                .attrs
                .iter()
                .filter(|a| a.path().is_ident("inject"))
                .last()
            {
                Some(a) => Some(a.parse_args::<InjectHelperExpr>().unwrap()),
                None => None,
            }
        })
        .collect::<Vec<_>>();
    let generic_params = &input.generics.params.iter().collect::<Vec<&GenericParam>>();
    let generic_keys = &generic_params
        .iter()
        .map(|p| match p {
            GenericParam::Type(t) => {
                let identity = &t.ident;
                quote! { #identity }
            }
            GenericParam::Const(c) => {
                let identity = &c.ident;
                quote! { #identity }
            }
            GenericParam::Lifetime(l) => quote! { #l },
        })
        .collect::<Vec<_>>();
    let lifetime_keys = &generic_params
        .iter()
        .filter_map(|p| match p {
            GenericParam::Lifetime(l) => Some(quote! { #l }),
            _ => None,
        })
        .collect::<Vec<_>>();
    let prov_lifetimes = match lifetime_keys.len() > 0 {
        true => quote! { 'prov: #(#lifetime_keys)+*, },
        false => quote! {},
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
            let items = types.iter().zip(&attributes).map(|(_, a)| match a {
                Some(attr) => {
                    let inputs = attr
                        .1
                        .iter()
                        .map(|x| quote! { let #x = provider.provide(); })
                        .collect::<Vec<_>>();
                    let output = &attr.0;
                    if inputs.is_empty() {
                        quote! { #output }
                    } else {
                        quote! {
                            {
                                #(#inputs)*
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
            let items = keys.iter().zip(&attributes).map(|(k, a)| match a {
                Some(attr) => {
                    let inputs = attr
                        .1
                        .iter()
                        .map(|x| quote! { let #x = provider.provide(); })
                        .collect::<Vec<_>>();
                    let output = &attr.0;
                    quote! {
                        #k: {
                            #(#inputs)*
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
        if let Some(attr) = a {
            for attr_type in attr.1.iter().map(|x| &x.bounds) {
                prov_types.push(quote! {#attr_type});
            }
        } else {
            prov_types.push(quote! {#t});
        }
    }
    prov_types.dedup_by(|a, b| a.to_string() == b.to_string());
    let output = quote! {
        #[derive(nject::InjectableHelperAttr)]
        #input

        impl<'prov, #(#generic_params,)*NjectProvider> nject::Injectable<'prov, #ident<#(#generic_keys),*>, NjectProvider> for #ident<#(#generic_keys),*>
            where
                #prov_lifetimes
                NjectProvider: #(nject::Provider<'prov, #prov_types>)+*,#where_predicates
        {
            #[inline]
            fn inject(provider: &'prov NjectProvider) -> #ident<#(#generic_keys),*> {
                #creation_output
            }
        }
    };
    output.into()
}
