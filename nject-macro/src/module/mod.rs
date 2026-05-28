use crate::core::{DeriveInput, FactoryExpr, FieldFactoryExpr, collection::group_by, error};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{
    Expr, GenericParam, PatType, Path, Token, Type,
    parse::{Parse, ParseStream},
    spanned::Spanned,
};

#[derive(Clone)]
enum ExportStructInput {
    TypeExpr(Type, Box<Expr>),
    TypeExprFact(Type, Vec<PatType>, Box<Expr>),
}
impl Parse for ExportStructInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let parsed_type = input.parse()?;
        input.parse::<Token![,]>()?;
        if input.peek(Token![|]) {
            let expr = FactoryExpr::parse(input)?;
            Ok(Self::TypeExprFact(parsed_type, expr.inputs, expr.body))
        } else {
            Ok(Self::TypeExpr(parsed_type, input.parse()?))
        }
    }
}

type ExportFieldInput = FieldFactoryExpr;

pub(crate) fn handle_module(attr: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    let input = syn::parse::<DeriveInput>(item)?;
    // Parse the module path attribute if present (kept for backward compatibility)
    let _module_pub_path = match attr.is_empty() {
        true => None,
        false => {
            let path = syn::parse::<Path>(attr).map_err(|e| {
                error::combine(syn::Error::new(e.span(), "Invalid public module path"), e)
            })?;
            Some(path)
        }
    };
    let ident = &input.ident;
    let fields = input.fields().iter().collect::<Vec<_>>();
    let struct_exports = input
        .attrs
        .iter()
        .filter(|a| a.path().is_ident("export"))
        .map(|a| {
            a.parse_args::<ExportStructInput>().map_err(|e| {
                error::combine(
                    syn::Error::new(a.span(), "Unable to parse struct export attribute."),
                    e,
                )
            })
        })
        .collect::<syn::Result<Vec<_>>>()?;
    let export_attr_indexes = fields
        .iter()
        .enumerate()
        .filter_map(|(i, f)| {
            let attrs = f
                .attrs
                .iter()
                .filter(|a| a.path().is_ident("export"))
                .collect::<Vec<_>>();
            if attrs.is_empty() {
                None
            } else {
                Some((i, attrs))
            }
        })
        .collect::<Vec<_>>();
    let struct_exports_by_type = group_by(struct_exports.iter(), |k| match k {
        ExportStructInput::TypeExpr(t, _) => quote! {#t}.to_string(),
        ExportStructInput::TypeExprFact(t, _, _) => quote! {#t}.to_string(),
    });
    let struct_type_exports = struct_exports
        .iter()
        .map(|e| match e {
            ExportStructInput::TypeExpr(t, _) => t,
            ExportStructInput::TypeExprFact(t, _, _) => t,
        })
        .collect::<Vec<_>>();

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
    let generic_params = input.generic_params();
    let struct_export_outputs = struct_exports_by_type
        .values()
        .map(|exports| {
            let values = exports.iter().map(|e| {
                let (mut ty, inputs, value) = match e.to_owned().to_owned() {
                    ExportStructInput::TypeExpr(t, v) => (t, vec![], v),
                    ExportStructInput::TypeExprFact(t, i, v) => (t, i, v),
                };
                super::core::substitute_in_type(&mut ty, "Self", ident.to_string().as_str());
                (ty, inputs, value)
            }).collect::<Vec<_>>();
            let iter_match_outputs = values.iter().enumerate().map(|(index,(_, inputs, value))| {
                quote! {
                    #index => {
                        #(let #inputs = provider.provide();)*
                        #value
                    },
                }
            });
            let prov_types = values.iter().flat_map(|(_, inputs, _)| inputs.iter().map(|i| &i.ty));
            let ty = &values.first().unwrap().0;
            let iter_output = quote! {
                impl<'prov, #(#generic_params,)*NjectProvider> nject::RefIterable<'prov, #ty, NjectProvider> for #ident<#(#generic_keys),*>
                    where
                        #prov_lifetimes
                        NjectProvider: #(nject::Provider<'prov, #prov_types>)+*, #where_predicates
                {
                    #[inline]
                    fn inject(&'prov self, provider: &'prov NjectProvider, index: usize) -> #ty {
                        match index {
                            #( #iter_match_outputs )*
                            _ => unreachable!("Invalid index {index}"),
                        }
                    }
                }
            };

            let (ty, inputs, value) = values.last().unwrap();
            let prov_types = inputs.iter().map(|i| &i.ty);
            quote!{

                impl<'prov, #(#generic_params,)*NjectProvider> nject::RefInjectable<'prov, #ty, NjectProvider> for #ident<#(#generic_keys),*>
                    where
                        #prov_lifetimes
                        NjectProvider: #(nject::Provider<'prov, #prov_types>)+*, #where_predicates
                {
                    #[inline]
                    fn inject(&'prov self, provider: &'prov NjectProvider) -> #ty {
                        #(let #inputs = provider.provide();)*
                        #value
                    }
                }

                #iter_output
            }
        });

    let export_outputs = export_attr_indexes.iter().map(|(i, attrs)| {
        let field = fields[*i];
        let ref_prefix = if let Type::Reference(r) = &field.ty {
            let lifetime = &r.lifetime;
             quote! { &#lifetime }
        } else {
            quote! { &'prov }
        };
        let inputs = attrs.iter().map(|a| match a.meta {
            syn::Meta::Path(_) => ExportFieldInput::Type(field.ty.to_owned()),
            _ => a.parse_args::<ExportFieldInput>().unwrap()
        });
        let index = syn::Index::from(*i);
    	let field_key = match &field.ident {
    		Some(i) => quote!{ #i },
    		None => quote!{ #index },
    	};
        let outputs = inputs.map(|input| {
            let ty = match &input {
                ExportFieldInput::None =>  match &field.ty {
                    Type::Reference(r) => {
                        let inner_ty = &r.elem;
                        quote! { #ref_prefix #inner_ty }
                    },
                    _ => {
                        let ty = &field.ty;
                        quote! { #ref_prefix #ty }
                    },
                },
                ExportFieldInput::Type(t) => match t {
                    Type::Reference(r) => {
                        let inner_ty = &r.elem;
                        quote! { #ref_prefix #inner_ty }
                    },
                    _ => quote! { #ref_prefix #t },
                },
                ExportFieldInput::TypeExpr(t, _, _) => quote! { #t },
            };

            let body = match &input {
                ExportFieldInput::TypeExpr(_, i, e) => {
                    let ref_prefix = match & field.ty {
                        Type::Reference(_) => quote!{},
                        _ => quote!{&},
                    };
                    quote!{
                        let #i = #ref_prefix provider.reference(). #field_key;
                        #e
                    }
                },
                _ => quote!{ & provider.reference(). #field_key }
            };
            quote! {
                #[allow(non_local_definitions)]
                impl<'prov, #(#generic_params,)*NjectProvider> nject::Injectable<'prov, #ty, NjectProvider> for #ty
                    where
                        #prov_lifetimes
                        NjectProvider: nject::Import<#ident<#(#generic_keys),*>>, #where_predicates
                {
                    #[inline]
                    fn inject(provider: &'prov NjectProvider) -> #ty {
                        #body
                    }
                }
            }
        });
        quote!{
            #(#outputs)*
        }
    });

    // Generate the macro_rules! for this module
    let module_macro_output = gen_module_macro(ident, &generic_params, &struct_type_exports);

    let output = quote! {
        #[derive(nject::ModuleHelperAttr)]
        #input
        #(#struct_export_outputs)*
        #(#export_outputs)*
        #module_macro_output
    };
    Ok(output.into())
}

/// Generate the `#[macro_export] macro_rules! __nject_module_{Ident}` macro
/// that participates in the chaining protocol.
///
/// The macro name is based solely on the struct name. This means module struct names
/// must be unique within a single crate.
fn gen_module_macro(
    ident: &syn::Ident,
    generic_params: &[&GenericParam],
    struct_type_exports: &[&Type],
) -> TokenStream2 {
    let macro_name = format_ident!("__nject_module_{}", ident);

    // To emit $ in macro_rules! from a proc macro, we use a dollar-sign token
    let dollar = proc_macro2::Punct::new('$', proc_macro2::Spacing::Alone);

    // Build the module_args pattern and the export entries
    let module_args_pattern = if generic_params.is_empty() {
        quote! {}
    } else {
        let patterns: Vec<TokenStream2> = generic_params
            .iter()
            .enumerate()
            .map(|(i, p)| {
                let var_name = format_ident!("__nject_g{}", i);
                let d = &dollar;
                match p {
                    GenericParam::Lifetime(_) => quote! { #d #var_name:lifetime },
                    GenericParam::Type(_) => quote! { #d #var_name:ty },
                    GenericParam::Const(_) => quote! { #d #var_name:expr },
                }
            })
            .collect();
        quote! { #(#patterns),* }
    };

    // Build the export type tokens
    let export_entries: Vec<TokenStream2> = struct_type_exports
        .iter()
        .map(|ty| {
            let d = &dollar;
            quote! { { field = [#d(#d __nject_field)*], ty = [#ty] } }
        })
        .collect();

    let d = &dollar;

    let exports_addition = if export_entries.is_empty() {
        quote! { #d(#d __nject_exports)* }
    } else {
        quote! {
            #d(#d __nject_exports)*
            #(#export_entries)*
        }
    };

    quote! {
        #[allow(non_local_definitions)]
        #[doc(hidden)]
        #[macro_export]
        macro_rules! #macro_name {
            (
                @nject_collect
                next = [#d(#d __nject_next:tt)*],
                field = [#d(#d __nject_field:tt)*],
                module_args = [#module_args_pattern],
                exports = [#d(#d __nject_exports:tt)*],
                #d(#d __nject_provider_info:tt)*
            ) => {
                ::nject::__nject_next! {
                    next = [#d(#d __nject_next)*],
                    exports = [
                        #exports_addition
                    ],
                    #d(#d __nject_provider_info)*
                }
            };
        }
    }
}

