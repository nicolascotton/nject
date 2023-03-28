use quote::quote;
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
