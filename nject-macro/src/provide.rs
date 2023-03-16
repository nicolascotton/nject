use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, DeriveInput, Expr, GenericParam, Token, Type,
};

struct TypeExpr(Type, Expr);
impl Parse for TypeExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let parsed_type = input.parse()?;
        input.parse::<Token![,]>()?;
        let parsed_value: Expr = input.parse()?;
        return Ok(TypeExpr(parsed_type, parsed_value));
    }
}

pub(crate) fn handle_provide(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attributes: TypeExpr = syn::parse(attr).unwrap();
    let input = parse_macro_input!(item as DeriveInput);
    let ident = &input.ident;
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
    let where_predicates = match &input.generics.where_clause {
        Some(w) => {
            let predicates = &w.predicates;
            quote! { #predicates }
        }
        None => quote! {},
    };
    let output_type = &attributes.0;
    let output_value = &attributes.1;
    let output = quote! {
        #input

        impl<'prov, #(#generic_params),*> nject::Provider<'prov, #output_type> for #ident<#(#generic_keys),*>
            where #where_predicates
        {
            #[inline]
            fn provide(&'prov self) -> #output_type {
                #output_value
            }
        }
    };
    output.into()
}
