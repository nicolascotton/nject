use super::models::{Provider, ProviderKey};
use std::{collections::HashMap, sync::RwLock};

thread_local! {
    static CACHE: RwLock<HashMap<ProviderKey, Provider>> = RwLock::new(HashMap::new());
}

fn update_cache(update: impl FnOnce(&mut HashMap<ProviderKey, Provider>)) {
    CACHE.with(move |cache| {
        let mut cache = cache.write().unwrap();
        update(&mut cache);
    });
}

pub(crate) fn get(key: &ProviderKey) -> Option<Provider> {
    CACHE.with(move |cache| {
        let cache = cache.read().unwrap();
        match cache.get(key) {
            Some(v) => Some(v.clone()),
            None => None,
        }
    })
}

pub(crate) fn add(key: ProviderKey, provider: Provider) {
    update_cache(|x| {
        x.insert(key, provider);
    });
}
