/// Combines 2 errors together.
pub fn combine(mut err: syn::Error, other: syn::Error) -> syn::Error {
    err.combine(other);
    err
}
