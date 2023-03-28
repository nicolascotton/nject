use super::models::ProvidedType;
use quote::quote;
use std::{collections::HashMap, sync::RwLock};
use syn::{Ident, Type};

thread_local! {
    static CACHE: RwLock<HashMap<String, Vec<ProvidedType>>> = RwLock::new(HashMap::new());
}

fn update_cache(update: impl FnOnce(&mut HashMap<String, Vec<ProvidedType>>)) {
    CACHE.with(move |cache| {
        let mut cache = cache.write().unwrap();
        update(&mut cache);
    });
}

fn get(key: &str) -> Vec<ProvidedType> {
    CACHE.with(move |cache| {
        let cache = cache.read().unwrap();
        match cache.get(key) {
            Some(v) => v.clone(),
            None => Vec::new(),
        }
    })
}

pub(crate) fn get_for_type(key: &Type) -> Vec<ProvidedType> {
    let key = super::extract_key_from_type(key);
    get(&key)
}

pub(crate) fn add(key: &Ident, provided_type: &Type) {
    let provided_type = ProvidedType::from(provided_type);
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
