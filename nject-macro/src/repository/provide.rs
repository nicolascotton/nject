use proc_macro::TokenStream;
use quote::quote;
use std::{collections::HashMap, sync::Mutex};
use syn::{Ident, Path, Type};

thread_local! {
    static CACHE: Mutex<HashMap<String, Vec<ProvidedType>>> = Mutex::new(HashMap::new());
}

fn update_cache(update: impl FnOnce(&mut HashMap<String, Vec<ProvidedType>>)) {
    CACHE.with(move |cache| {
        let mut cache = cache.lock().unwrap();
        update(&mut cache);
    });
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

fn get(key: &str) -> Vec<ProvidedType> {
    CACHE.with(move |cache| {
        let cache = cache.lock().unwrap();
        match cache.get(key) {
            Some(v) => v.clone(),
            None => Vec::new(),
        }
    })
}

pub(crate) fn get_for_type(key: &Type) -> Vec<ProvidedType> {
    let key = extract_key_from_type(key);
    get(&key)
}

pub(crate) fn add(key: &Ident, provided_type: &Type) {
    let provided_type = ProvidedType {
        ty: quote! {#provided_type}.to_string(),
    };
    let key = quote! {#key}.to_string();
    update_cache(|x| {
        let types = x.entry(key).or_default();
        types.push(provided_type);
    });
}

pub(crate) fn remove(key: &Ident) {
    let key = quote! {#key}.to_string();
    update_cache(|x| {
        x.remove(&key);
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
