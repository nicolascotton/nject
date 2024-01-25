use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::ops::Deref;
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
    Expr, ExprClosure, Fields, GenericParam, Ident, Pat, PatType, Type,
};

pub struct DeriveInput(syn::DeriveInput);

impl Deref for DeriveInput {
    type Target = syn::DeriveInput;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Parse for DeriveInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let input = input.parse()?;
        Ok(Self(input))
    }
}

impl ToTokens for DeriveInput {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens)
    }
}

impl DeriveInput {
    pub fn fields(&self) -> &Fields {
        match &self.data {
            syn::Data::Struct(d) => &d.fields,
            _ => panic!("Unsupported type. Macro should be used on a struct"),
        }
    }
    pub fn field_types(&self) -> Vec<&Type> {
        self.fields().iter().map(|f| &f.ty).collect::<Vec<_>>()
    }
    pub fn field_idents(&self) -> Vec<&Ident> {
        self.fields()
            .iter()
            .map(|f| f.ident.as_ref())
            .filter_map(|i| i)
            .collect::<Vec<_>>()
    }
    pub fn generic_params(&self) -> Vec<&GenericParam> {
        self.generics.params.iter().collect::<Vec<_>>()
    }
    pub fn generic_keys(&self) -> Vec<TokenStream> {
        self.generics
            .params
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
            .collect::<Vec<_>>()
    }
    pub fn lifetime_keys(&self) -> Vec<TokenStream> {
        self.generics
            .params
            .iter()
            .filter_map(|p| match p {
                GenericParam::Lifetime(l) => Some(quote! { #l }),
                _ => None,
            })
            .collect::<Vec<_>>()
    }
}

pub struct FactoryExpr {
    pub inputs: Vec<PatType>,
    pub body: Box<Expr>,
}

impl Parse for FactoryExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let expr: ExprClosure = input.parse()?;
        let mut inputs = Vec::with_capacity(expr.inputs.len());
        let span = expr.span();
        for input in expr.inputs {
            if let Pat::Type(pat_type) = input {
                inputs.push(pat_type);
            } else {
                return Err(syn::Error::new(
                    span,
                    format!("Invalid input: {}", input.to_token_stream()),
                ));
            }
        }
        Ok(FactoryExpr {
            inputs,
            body: expr.body,
        })
    }
}
