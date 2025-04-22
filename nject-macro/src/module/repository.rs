use super::models::{Module, ModuleKey};
use crate::core::{
    cache_path,
    encoding::{base32, base64},
    hash::fnv,
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
    if let Ok(dir) = std::fs::read_dir(cache_root_dir) {
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
            let file_path = file.path();
            if let Ok(file) = std::fs::File::open(&file_path) {
                let lines = std::io::BufReader::new(file).lines();
                let lines = lines.map_while(Result::ok).collect::<Vec<String>>();
                let crate_name = lines.first().expect("Missing crate name field").to_owned();
                let bin_name = lines.get(1).expect("Missing bin name field").to_owned();
                let path = lines.get(2).expect("Missing path field").to_owned();
                let exported_types = lines.iter().skip(3).map(|x| x.to_owned()).collect();
                let module = Module {
                    crate_name: match crate_name.is_empty() {
                        true => None,
                        false => Some(crate_name),
                    },
                    bin_name: match bin_name.is_empty() {
                        true => None,
                        false => Some(bin_name),
                    },
                    path,
                    exported_types,
                };
                match module.key() {
                    Ok(key) => _ = cache.insert(key, module),
                    Err(_) => _ = std::fs::remove_file(&file_path), // The file has an invalid key. We can remove it.
                };
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
        cache.get(key).cloned()
    })
}

/// Ensure a module for the given key.
pub(crate) fn ensure(module: Module) {
    let module_dir = cache_path();
    let key = module.key().expect("Unable to construct module key.");
    // OS have limits on file name size. If the name is too long, we use a portion of the original key with a hash.
    let module_file_name = to_file_name(key.0.as_bytes());
    let module_path = module_dir.join(module_file_name);
    let no_export_types = module.exported_types.is_empty();
    let mut exported_types_output = Vec::<u8>::new();
    if let Some(crate_name) = &module.crate_name {
        exported_types_output.extend(crate_name.as_bytes());
    }
    exported_types_output.push(b'\n');
    if let Some(bin_name) = &module.bin_name {
        exported_types_output.extend(bin_name.as_bytes());
    }
    exported_types_output.push(b'\n');
    exported_types_output.extend(module.path.as_bytes());
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

/// Sanitize bytes into an encoded file name.
fn to_file_name(data: &[u8]) -> String {
    let encoded = if data.len() > 40 {
        let hash = fnv(data);
        let prefix = &data[..32];
        let mut combined = Vec::with_capacity(48);
        combined.extend_from_slice(prefix);
        combined.extend_from_slice(&hash);
        let mut encoded_data = base64::encode(&combined);
        // '/' is not a valid file name char
        for byte in &mut encoded_data {
            if *byte == b'/' {
                *byte = b'_';
            }
        }
        encoded_data
    } else {
        base32::encode(data) // base32 is safe without hash on windows since it does not have a mix of lower/upper cases.
    };
    unsafe { String::from_utf8_unchecked(encoded) } // base64 & base32 are valid utf-8 bytes.
}
