#![doc = include_str!("../../README.md")]
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, DeriveInput, Expr, ExprParen, GenericParam, Ident, Token, Type,
};

/// For internal purposes only. Should not be used.
#[proc_macro_derive(InjectableHelperAttr, attributes(inject))]
pub fn injectable_helper_attr(_item: TokenStream) -> TokenStream {
    TokenStream::new()
}

/// Attribute to mark a struct as injectable.
/// ```rust
/// use nject_macro::{injectable, provider};
///
/// #[injectable]
/// struct Facade;
///
/// #[provider]
/// struct Provider;
///
/// fn main() {
///     let _facade: Facade = Provider.provide();
/// }
/// ```
#[proc_macro_attribute]
pub fn injectable(_attr: TokenStream, item: TokenStream) -> TokenStream {
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
        .map(
            |f| match f.attrs.iter().filter(|a| a.path.is_ident("inject")).last() {
                Some(a) => Some(syn::parse2::<ExprParen>(a.tokens.clone()).unwrap().expr),
                None => None,
            },
        )
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
                Some(attr) => quote! { #attr },
                None => quote! { provider.provide() },
            });
            quote! { #ident(#(#items),*) }
        }
        false => {
            let items = keys.iter().zip(&attributes).map(|(k, a)| match a {
                Some(attr) => quote! { #k: #attr },
                None => quote! { #k: provider.provide() },
            });
            quote! { #ident { #(#items),* } }
        }
    };
    let prov_types = types
        .iter()
        .zip(&attributes)
        .filter_map(|(t, a)| match a {
            Some(_) => None,
            None => Some(t),
        })
        .collect::<Vec<_>>();
    let output = quote! {
        #[derive(nject::InjectableHelperAttr)]
        #input

        impl<'prov, #(#generic_params,)*NjectProvider> nject::Injectable<'prov, #ident<#(#generic_keys),*>, NjectProvider> for #ident<#(#generic_keys),*>
            where
                #prov_lifetimes
                NjectProvider: #(nject::Provider<'prov, #prov_types>)+*,#where_predicates
        {
            fn inject(provider: &'prov NjectProvider) -> #ident<#(#generic_keys),*> {
                #creation_output
            }
        }
    };
    output.into()
}

/// Attribute to specify a desired injected value.
/// ```rust
/// use nject_macro::{inject, injectable, provider};
///
/// #[inject(Self { value: 42 })]
/// struct DepOne {
///     value: i32,
/// }
///
/// #[injectable]
/// struct Facade(DepOne, #[inject(123)] i32);
///
/// #[provider]
/// struct Provider;
///
/// fn main() {
///     let _facade: Facade = Provider.provide();
/// }
/// ```
#[proc_macro_attribute]
pub fn inject(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let creation_output = parse_macro_input!(attr as Expr);
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
    let output = quote! {
        #input

        impl<'prov, #(#generic_params,)*NjectProvider> nject::Injectable<'prov, #ident<#(#generic_keys),*>, NjectProvider> for #ident<#(#generic_keys),*>
            where
                #prov_lifetimes
                #where_predicates
        {
            fn inject(provider: &'prov NjectProvider) -> #ident<#(#generic_keys),*> {
                #creation_output
            }
        }
    };
    output.into()
}

/// Attribute to mark a struct as a provider.
/// ```rust
/// use nject_macro::{injectable, provider};
///
/// #[injectable]
/// struct Facade;
///
/// #[provider]
/// struct Provider;
///
/// fn main() {
///     let _facade: Facade = Provider.provide();
/// }
/// ```
#[proc_macro_attribute]
pub fn provider(_attr: TokenStream, item: TokenStream) -> TokenStream {
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
    let output = quote! {
        #input

        impl<'prov, #(#generic_params,)*Njecty> nject::Provider<'prov, Njecty> for #ident<#(#generic_keys),*>
            where Njecty: nject::Injectable<'prov, Njecty, #ident<#(#generic_keys),*>>,#where_predicates
        {
            fn provide(&'prov self) -> Njecty {
                Njecty::inject(self)
            }
        }

        impl<#(#generic_params),*> #ident<#(#generic_keys),*>
            where #where_predicates
        {
            pub fn provide<'prov, Njecty>(&'prov self) -> Njecty
                where Self: nject::Provider<'prov, Njecty>
            {
                <Self as nject::Provider<'prov, Njecty>>::provide(self)
            }
        }
    };
    output.into()
}

struct TypeExpr(Type, Expr);
impl Parse for TypeExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let parsed_type = input.parse().unwrap();
        input.parse::<Token![,]>().unwrap();
        let parsed_value: Expr = input.parse().unwrap();
        return Ok(TypeExpr(parsed_type, parsed_value));
    }
}

/// Attribute to provide a given instance for a specific type.
/// ```rust
/// use nject_macro::{injectable, provide, provider};
///
/// struct Dependency {
///     value: i32,
/// }
///
/// #[injectable]
/// struct Facade(Dependency);
///
/// #[provider]
/// #[provide(Dependency, Dependency { value: 123 })]
/// struct Provider;
///
/// fn main() {
///     let _dependency: Dependency = Provider.provide();
///     let _facade: Facade = Provider.provide();
/// }
/// ```
#[proc_macro_attribute]
pub fn provide(attr: TokenStream, item: TokenStream) -> TokenStream {
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
            fn provide(&'prov self) -> #output_type {
                #output_value
            }
        }
    };
    output.into()
}
