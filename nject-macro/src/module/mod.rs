pub mod models;
pub mod repository;
use crate::core::{collection::group_by, error, DeriveInput, FactoryExpr, FieldFactoryExpr};
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
    Expr, PatType, Path, Token, Type,
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
    let module_pub_path = match attr.is_empty() {
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
    let module = models::Module::from((
        ident,
        module_pub_path.as_ref(),
        struct_type_exports.as_slice(),
    ));
    repository::ensure(module);
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

    let output = quote! {
        #[derive(nject::ModuleHelperAttr)]
        #input
        #(#struct_export_outputs)*
        #(#export_outputs)*
    };
    Ok(output.into())
}
