use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, GenericParam, Ident, Type};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct ProviderKey(pub(crate) String);

impl From<&Ident> for ProviderKey {
    fn from(value: &Ident) -> Self {
        Self(quote! {#value}.to_string())
    }
}

impl From<&Type> for ProviderKey {
    fn from(value: &Type) -> Self {
        Self(super::extract_key_from_type(value))
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Provider {
    generic_keys: Vec<String>,
}

impl Provider {
    pub(crate) fn generic_substitution_map(&self, other_prov: &Provider) -> Vec<(String, String)> {
        self.generic_keys
            .iter()
            .zip(other_prov.generic_keys.iter())
            .map(|(from, to)| (from.to_owned(), to.to_owned()))
            .collect()
    }
}

impl From<&DeriveInput> for Provider {
    fn from(value: &DeriveInput) -> Self {
        Provider {
            generic_keys: value
                .generics
                .params
                .iter()
                .map(|p| match p {
                    GenericParam::Type(t) => {
                        let identity = &t.ident;
                        quote! {#identity}.to_string()
                    }
                    GenericParam::Const(c) => {
                        let identity = &c.ident;
                        quote! {#identity}.to_string()
                    }
                    GenericParam::Lifetime(l) => quote! {#l}.to_string(),
                })
                .collect::<Vec<_>>(),
        }
    }
}

impl From<&Type> for Provider {
    fn from(value: &Type) -> Self {
        let path = super::extract_path_from_type(value);
        let last_segment = path
            .segments
            .last()
            .expect("Type must have at least one segment.");
        let generics = match &last_segment.arguments {
            syn::PathArguments::None => None,
            syn::PathArguments::AngleBracketed(a) => Some(a),
            syn::PathArguments::Parenthesized(_) => panic!("Unsupported provide type."),
        };
        if let Some(generics) = generics {
            Provider {
                generic_keys: generics
                    .args
                    .iter()
                    .map(|g| {
                        let identity = g;
                        quote! {#identity}.to_string()
                    })
                    .collect(),
            }
        } else {
            Provider {
                generic_keys: vec![],
            }
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct ProvidedType {
    ty: String,
}

impl ProvidedType {
    pub(crate) fn to_type(&self) -> Type {
        let ty: TokenStream = self
            .ty
            .parse()
            .expect("ProvidedType to be parsable into a TokenStream");
        syn::parse(ty).expect("ProvidedType to be parsable into a Type.")
    }

    pub(crate) fn to_type_with_generics_substitutions(&self, subs: &Vec<(String, String)>) -> Type {
        if let Some((name, generics)) = self.ty.split_once(' ') {
            let mut new_generics = String::from(generics);
            for (from, to) in subs {
                new_generics = new_generics.replace(from, to);
            }
            let ty: TokenStream = format!("{name} {new_generics}")
                .parse()
                .expect("ProvidedType to be parsable into a TokenStream");
            syn::parse(ty).expect("ProvidedType to be parsable into a Type.")
        } else {
            self.to_type()
        }
    }
}

impl From<&Type> for ProvidedType {
    fn from(value: &Type) -> Self {
        ProvidedType {
            ty: quote! {#value}.to_string(),
        }
    }
}
