use quote::quote;
use std::{collections::HashMap, sync::Mutex};
use syn::{DeriveInput, GenericParam, Path, Type};

thread_local! {
    static CACHE: Mutex<HashMap<String, Provider>> = Mutex::new(HashMap::new());
}

fn update_cache(update: impl FnOnce(&mut HashMap<String, Provider>)) {
    CACHE.with(move |cache| {
        let mut cache = cache.lock().unwrap();
        update(&mut cache);
    });
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
        let path = extract_path_from_type(value);
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

fn get(key: &str) -> Option<Provider> {
    CACHE.with(move |cache| {
        let cache = cache.lock().unwrap();
        match cache.get(key) {
            Some(v) => Some(v.clone()),
            None => None,
        }
    })
}

pub(crate) fn get_for_type(key: &Type) -> Option<Provider> {
    let key = extract_key_from_type(key);
    get(&key)
}

pub(crate) fn add(provider: &DeriveInput) {
    let ident = &provider.ident;
    let provider = Provider::from(provider);
    let key = quote! {#ident}.to_string();
    update_cache(|x| {
        x.insert(key, provider);
    });
}

fn extract_key_from_type(ty: &Type) -> String {
    let path = extract_path_from_type(ty);
    let segment = path
        .segments
        .last()
        .expect("Path must at least have one segment");
    let segment_ident = &segment.ident;
    quote! {#segment_ident}.to_string()
}

fn extract_path_from_type(ty: &Type) -> &Path {
    match ty {
        Type::Path(p) => &p.path,
        Type::Reference(r) => extract_path_from_type(&r.elem),
        _ => panic!("Unsupported type. Must be a Path or a Reference type."),
    }
}
