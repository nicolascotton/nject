use super::models::Provider;
use quote::quote;
use std::{collections::HashMap, sync::RwLock};
use syn::{DeriveInput, Type};

thread_local! {
    static CACHE: RwLock<HashMap<String, Provider>> = RwLock::new(HashMap::new());
}

fn update_cache(update: impl FnOnce(&mut HashMap<String, Provider>)) {
    CACHE.with(move |cache| {
        let mut cache = cache.write().unwrap();
        update(&mut cache);
    });
}

fn get(key: &str) -> Option<Provider> {
    CACHE.with(move |cache| {
        let cache = cache.read().unwrap();
        match cache.get(key) {
            Some(v) => Some(v.clone()),
            None => None,
        }
    })
}

pub(crate) fn get_for_type(key: &Type) -> Option<Provider> {
    let key = super::extract_key_from_type(key);
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
