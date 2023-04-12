use super::{
    cache_path,
    models::{ProvidedType, ProviderKey},
};
use std::{collections::HashMap, io::Write, sync::RwLock};

fn init_cache() -> HashMap<ProviderKey, Vec<ProvidedType>> {
    let mut cache: HashMap<ProviderKey, Vec<ProvidedType>> = HashMap::new();
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
            if let Ok(provide_dir) = std::fs::read_dir(dir.join("provides")) {
                let inner_files = provide_dir.filter_map(|x| match x {
                    Ok(e) => match e.file_type() {
                        Ok(t) => {
                            if t.is_file() {
                                Some(e.path())
                            } else {
                                None
                            }
                        }
                        Err(_) => None,
                    },
                    Err(_) => None,
                });
                for file in inner_files {
                    let file_name = file
                        .file_name()
                        .expect("Provided type to have a name.")
                        .to_str()
                        .expect("Provided type to be a valid str.");
                    let provided_type = decode(file_name);
                    let provided_types = cache
                        .entry(ProviderKey(provider_id.to_owned()))
                        .or_default();
                    provided_types.push(ProvidedType { ty: provided_type });
                }
            }
        }
    }
    cache
}

thread_local! {
    static CACHE: RwLock<HashMap<ProviderKey, Vec<ProvidedType>>> = RwLock::new(init_cache());
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
    let provides_dir = cache_path().join(&key.0).join("provides");
    let ty_hex = encode(&provided_type.ty);
    let ty_path = provides_dir.join(ty_hex);
    update_cache(|x| {
        let types = x.entry(key).or_default();
        types.push(provided_type);
    });
    super::retry(10, || std::fs::create_dir_all(&provides_dir))
        .expect("Unable to create providers directory.");
    super::retry(10, || {
        let path = ty_path.to_owned();
        if path.exists() {
            Ok::<(), std::io::Error>(())
        } else {
            std::fs::File::create(&path)?;
            Ok(())
        }
    })
    .expect("Unable to create provided type file.");
}

pub(crate) fn remove(key: &ProviderKey) {
    let provider_dir = cache_path().join(&key.0);
    update_cache(|x| {
        x.remove(key);
    });
    super::retry(10, || {
        let path = provider_dir.join("provides");
        match path.exists() {
            true => std::fs::remove_dir_all(&path),
            false => Ok(()),
        }
    })
    .expect("Unable to remove provides directory.");
}

fn encode(data: &str) -> String {
    let mut encoded_data = Vec::with_capacity(data.len() * 2);
    for d in data.as_bytes() {
        write!(&mut encoded_data, "{:X}", d).expect("Unable to encode data");
    }
    String::from_utf8(encoded_data).expect("Unable to encode data.")
}

fn decode(data: &str) -> String {
    let decoded_data = (0..data.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(data.get(i..i + 2).expect("Unable to decode data."), 16)
                .expect("Unable to decode data.")
        })
        .collect::<Vec<u8>>();
    String::from_utf8(decoded_data).expect("Unable to decode data.")
}
