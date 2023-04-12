use quote::quote;
use std::path::PathBuf;
use syn::{Path, Type};

pub mod models;
pub mod provide;
pub mod provider;

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

fn cache_path() -> PathBuf {
    let root_path = env!("NJECT_OUT_DIR");
    std::path::Path::new(root_path).join(".nject")
}

fn retry<T, E>(times: usize, action: impl Fn() -> Result<T, E>) -> Result<T, E> {
    let result = action();
    if result.is_ok() {
        result
    } else if times <= 0 {
        result
    } else {
        std::thread::sleep(std::time::Duration::from_millis(100));
        retry(times - 1, action)
    }
}
