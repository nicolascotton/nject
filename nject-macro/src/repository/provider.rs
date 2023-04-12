use super::{
    cache_path,
    models::{Provider, ProviderKey},
};
use std::{
    collections::HashMap,
    io::{BufRead, Write},
    sync::RwLock,
};

fn init_cache() -> HashMap<ProviderKey, Provider> {
    let mut cache = HashMap::new();
    let cache_root_dir = cache_path();
    if let Ok(dir) = std::fs::read_dir(&cache_root_dir) {
        let inner_dirs = dir.filter_map(|x| match x {
            Ok(e) => match e.file_type() {
                Ok(t) => {
                    if t.is_dir() {
                        Some(e.path())
                    } else {
                        None
                    }
                }
                Err(_) => None,
            },
            Err(_) => None,
        });
        for dir in inner_dirs {
            let provider_id = dir
                .file_name()
                .expect("Provider directory to have a name.")
                .to_str()
                .expect("Provider directory to be a valid str.");
            if let Ok(file) = std::fs::File::open(dir.join("generics")) {
                let lines = std::io::BufReader::new(file).lines();
                let generic_keys = lines
                    .filter_map(|l| match l {
                        Ok(g) => Some(g),
                        Err(_) => None,
                    })
                    .collect::<Vec<String>>();
                cache.insert(
                    ProviderKey(provider_id.to_owned()),
                    Provider { generic_keys },
                );
            }
        }
    }
    cache
}

thread_local! {
    static CACHE: RwLock<HashMap<ProviderKey, Provider>> = RwLock::new(init_cache());
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
    let provider_dir = cache_path().join(&key.0);
    let mut generics_output = Vec::<u8>::new();
    for gen in &provider.generic_keys {
        generics_output.extend(gen.as_bytes());
        generics_output.push(b'\n');
    }
    update_cache(|x| {
        x.insert(key, provider);
    });
    super::retry(10, || {
        if provider_dir.exists() {
            std::fs::remove_dir_all(&provider_dir)
        } else {
            Ok(())
        }
    })
    .expect("Unable to remove provider directory.");
    super::retry(10, || std::fs::create_dir_all(&provider_dir))
        .expect("Unable to create provider directory");
    super::retry(10, || {
        let mut file = std::fs::File::create(provider_dir.join("generics"))?;
        file.write_all(&generics_output)
    })
    .expect("Unable to create provider generics file");
}
