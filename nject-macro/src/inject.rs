use crate::core::{DeriveInput, FactoryExpr};
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Expr, PatType, Token,
};

struct InjectExpr(Box<Expr>, Vec<PatType>);
impl Parse for InjectExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Token![|]) {
            let expr = FactoryExpr::parse(input)?;
            Ok(InjectExpr(expr.body, expr.inputs))
        } else {
            Ok(InjectExpr(input.parse()?, vec![]))
        }
    }
}

pub(crate) fn handle_inject(item: TokenStream, attr: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let attributes: InjectExpr = syn::parse(attr).expect("Unable to parse inject attributes.");
    let ident = &input.ident;
    let generic_params = input.generic_params();
    let generic_keys = input.generic_keys();
    let lifetime_keys = input.lifetime_keys();
    let prov_lifetimes = match lifetime_keys.len() > 0 {
        true => quote! { 'prov: #(#lifetime_keys)+*, },
        false => quote! {},
    };
    let prov_types = attributes.1.iter().map(|x| &x.ty).collect::<Vec<_>>();
    let where_predicates = match &input.generics.where_clause {
        Some(w) => {
            let predicates = &w.predicates;
            quote! { #predicates }
        }
        None => quote! {},
    };
    let prov_input = attributes
        .1
        .iter()
        .map(|x| quote! { let #x = provider.provide(); })
        .collect::<Vec<_>>();
    let factory = attributes.0;
    let creation_output = quote! {
       #(#prov_input)*
       #factory
    };
    let output = quote! {
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
