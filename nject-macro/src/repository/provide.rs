use super::models::{ProvidedType, ProviderKey};
use std::{collections::HashMap, sync::RwLock};

thread_local! {
    static CACHE: RwLock<HashMap<ProviderKey, Vec<ProvidedType>>> = RwLock::new(HashMap::new());
}

fn update_cache(update: impl FnOnce(&mut HashMap<ProviderKey, Vec<ProvidedType>>)) {
    CACHE.with(move |cache| {
        let mut cache = cache.write().unwrap();
        update(&mut cache);
    });
}

pub(crate) fn get(key: &ProviderKey) -> Vec<ProvidedType> {
    CACHE.with(move |cache| {
        let cache = cache.read().unwrap();
        match cache.get(key) {
            Some(v) => v.clone(),
            None => Vec::new(),
        }
    })
}

pub(crate) fn add(key: ProviderKey, provided_type: ProvidedType) {
    update_cache(|x| {
        let types = x.entry(key).or_default();
        types.push(provided_type);
    });
}

pub(crate) fn remove(key: &ProviderKey) {
    update_cache(|x| {
        x.remove(key);
    });
}
