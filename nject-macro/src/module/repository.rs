use super::models::{Module, ModuleKey};
use crate::core::{
    cache_path,
    encoding::base32::{decode, encode},
    retry,
};
use std::{
    collections::HashMap,
    io::{BufRead, Write},
    sync::RwLock,
};

/// Initialize the cache with the file system.
fn init_cache() -> HashMap<ModuleKey, Module> {
    let mut cache = HashMap::new();
    let cache_root_dir = cache_path();
    if let Ok(dir) = std::fs::read_dir(&cache_root_dir) {
        let files = dir.filter_map(|x| match x {
            Ok(e) => match e.file_type() {
                Ok(t) => match t.is_file() {
                    true => Some(e),
                    false => None,
                },
                Err(_) => None,
            },
            Err(_) => None,
        });
        for file in files {
            let key = file
                .file_name()
                .to_str()
                .expect("Unable to get module key from file.")
                .to_owned();
            let key = decode(&key).expect("Invalid module file name.");
            if let Ok(file) = std::fs::File::open(file.path()) {
                let lines = std::io::BufReader::new(file).lines();
                let lines = lines
                    .filter_map(|l| match l {
                        Ok(g) => Some(g),
                        Err(_) => None,
                    })
                    .collect::<Vec<String>>();
                let crate_name = lines.get(0).expect("Missing crate name field").to_owned();
                let path = lines.get(1).expect("Missing path field").to_owned();
                let exported_types = lines.iter().skip(2).map(|x| x.to_owned()).collect();
                cache.insert(
                    ModuleKey(key),
                    Module {
                        crate_name: match crate_name.is_empty() {
                            true => None,
                            false => Some(crate_name),
                        },
                        path,
                        exported_types,
                    },
                );
            }
        }
    }
    cache
}

thread_local! {
    static CACHE: RwLock<HashMap<ModuleKey, Module>> = RwLock::new(init_cache());
}

/// Access the cache with the update callback.
fn update_cache<R>(update: impl FnOnce(&mut HashMap<ModuleKey, Module>) -> R) -> R {
    CACHE.with(move |cache| {
        let mut cache = cache.write().expect("Unable to gain cache access.");
        update(&mut cache)
    })
}

/// Get the module related to the given key.
/// If the module is not found, None is returned.
pub(crate) fn get(key: &ModuleKey) -> Option<Module> {
    CACHE.with(move |cache| {
        let cache = cache.read().expect("Unable to gain cache access.");
        match cache.get(key) {
            Some(v) => Some(v.clone()),
            None => None,
        }
    })
}

/// Ensure a module for the given key.
pub(crate) fn ensure(key: ModuleKey, module: Module) {
    let module_dir = cache_path();
    let encoded_key = encode(&key.0);
    let module_path = module_dir.join(&encoded_key);
    let no_export_types = module.exported_types.is_empty();
    let mut exported_types_output = Vec::<u8>::new();
    if let Some(crate_name) = &module.crate_name {
        exported_types_output.extend(crate_name.as_bytes());
    }
    exported_types_output.push(b'\n');
    exported_types_output.extend((&module.path).as_bytes());
    exported_types_output.push(b'\n');
    for ty in &module.exported_types {
        exported_types_output.extend(ty.as_bytes());
        exported_types_output.push(b'\n');
    }
    let updated = update_cache(|x| {
        if no_export_types {
            x.remove(&key).is_some()
        } else {
            match x.insert(key, module.clone()) {
                Some(prev) => !prev.eq(&module),
                None => true,
            }
        }
    });
    if !updated {
        return;
    }
    if no_export_types {
        retry(10, || match std::fs::remove_file(&module_path) {
            Ok(x) => Ok(x),
            Err(error) => match error.kind() {
                std::io::ErrorKind::NotFound => Ok(()),
                _ => Err(error),
            },
        })
        .expect("Unable to remove module file");
        return;
    }
    retry(10, || std::fs::create_dir_all(&module_dir)).expect("Unable to create module directory");
    retry(10, || {
        let mut file = std::fs::File::create(&module_path)?;
        file.write_all(&exported_types_output)
    })
    .expect("Unable to create module file");
}
