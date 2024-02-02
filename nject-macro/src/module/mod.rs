pub mod models;
pub mod repository;
use crate::core::{DeriveInput, FactoryExpr};
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Expr, PatType, Path, Token, Type,
};

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

pub(crate) fn handle_module(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let module_pub_path = match attr.is_empty() {
        true => None,
        false => Some(syn::parse::<Path>(attr).expect("Invalid public module path")),
    };
    let ident = &input.ident;
    let fields = input.fields().iter().collect::<Vec<_>>();
    let mut struct_exports = input
        .attrs
        .iter()
        .filter(|a| a.path().is_ident("export"))
        .map(|a| {
            a.parse_args::<ExportStructInput>()
                .expect("Unable to parse struct export attribute.")
        })
        .collect::<Vec<_>>();
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
    let module_key = module.key();
    repository::add(module_key, module);
    let generic_keys = input.generic_keys();
    let lifetime_keys = input.lifetime_keys();
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
    let generic_params = input.generic_params();
    let struct_export_outputs = struct_exports
        .iter_mut()
        .map(|t| {
            let mut empty = vec![];
            let (mut ty, inputs, value) = match t {
                ExportStructInput::TypeExpr(t, v) => (t,&mut empty, v),
                ExportStructInput::TypeExprFact(t, i, v) => (t, i, v),
            };
            super::core::substitute_in_type(&mut ty, "Self", ident.to_string().as_str());
            let prov_types = inputs.iter().map(|i| &i.ty);
            quote!{

                impl<'prov, #(#generic_params,)*NjectProvider> nject::RefInjectable<'prov, #ty, NjectProvider> for #ident<#(#generic_keys),*>
                    where
                        #prov_lifetimes
                        NjectProvider: #(nject::Provider<'prov, #prov_types>)+*,#where_predicates
                {
                    #[inline]
                    fn inject(&'prov self, provider: &'prov NjectProvider) -> #ty {
                        #(let #inputs = provider.provide();)*
                        #value
                    }
                }
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
        let attr_types = attrs.iter().map(|a| match a.meta {
            syn::Meta::Path(_) => field.ty.to_owned(),
            _ => a.parse_args::<Type>().unwrap()
        });
        let ty_outputs = attr_types.map(|ty| match ty {
            Type::Reference(r) => {
                let inner_ty = &r.elem;
                quote! { #ref_prefix #inner_ty }
            },
            _ => quote! { #ref_prefix #ty },
        });
        let index = syn::Index::from(*i);
    	let field_key = match &field.ident {
    		Some(i) => quote!{ #i },
    		None => quote!{ #index },
    	};
        let outputs = ty_outputs.map(|ty_output| quote! {
    		impl<'prov, #(#generic_params,)*NjectProvider> nject::Injectable<'prov, #ty_output, NjectProvider> for #ty_output
    			where
    				#prov_lifetimes
    				NjectProvider: nject::Import<#ident<#(#generic_keys),*>>,#where_predicates
    		{
    			#[inline]
    			fn inject(provider: &'prov NjectProvider) -> #ty_output {
        			&provider.reference().#field_key
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
    output.into()
}
