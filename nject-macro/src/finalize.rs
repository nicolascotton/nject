use crate::core::collection::group_by;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    Ident, Token, braced,
    parse::{Parse, ParseStream},
};

/// A single export entry: `{ field = [field_tokens], ty = [type_tokens] }`
struct ExportEntry {
    field: TokenStream2,
    ty: TokenStream2,
}

impl Parse for ExportEntry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        braced!(content in input);

        // field = [...]
        let field_ident: Ident = content.parse()?;
        if field_ident != "field" {
            return Err(syn::Error::new(field_ident.span(), "expected `field`"));
        }
        content.parse::<Token![=]>()?;
        let field_content;
        syn::bracketed!(field_content in content);
        let field: TokenStream2 = field_content.parse()?;

        // comma
        content.parse::<Token![,]>()?;

        // ty = [...]
        let ty_ident: Ident = content.parse()?;
        if ty_ident != "ty" {
            return Err(syn::Error::new(ty_ident.span(), "expected `ty`"));
        }
        content.parse::<Token![=]>()?;
        let ty_content;
        syn::bracketed!(ty_content in content);
        let ty: TokenStream2 = ty_content.parse()?;

        // optional trailing comma
        let _ = content.parse::<Token![,]>();

        Ok(ExportEntry { field, ty })
    }
}

/// The full input to `__nject_finalize_imports!`.
///
/// ```text
/// exports = [
///     { field = [...], ty = [...] }
///     ...
/// ],
/// provider_generics = [...],
/// provider_type = [...],
/// where_clause = [...],
/// fields_prefix = [...],
/// ```
struct FinalizeInput {
    exports: Vec<ExportEntry>,
    provider_generics: TokenStream2,
    provider_type: TokenStream2,
    where_clause: TokenStream2,
    fields_prefix: TokenStream2,
}

impl Parse for FinalizeInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // exports = [...]
        let exports_ident: Ident = input.parse()?;
        if exports_ident != "exports" {
            return Err(syn::Error::new(exports_ident.span(), "expected `exports`"));
        }
        input.parse::<Token![=]>()?;
        let exports_content;
        syn::bracketed!(exports_content in input);
        let mut exports = Vec::new();
        while !exports_content.is_empty() {
            exports.push(exports_content.parse::<ExportEntry>()?);
        }
        input.parse::<Token![,]>()?;

        // provider_generics = [...]
        let pg_ident: Ident = input.parse()?;
        if pg_ident != "provider_generics" {
            return Err(syn::Error::new(
                pg_ident.span(),
                "expected `provider_generics`",
            ));
        }
        input.parse::<Token![=]>()?;
        let pg_content;
        syn::bracketed!(pg_content in input);
        let provider_generics: TokenStream2 = pg_content.parse()?;
        input.parse::<Token![,]>()?;

        // provider_type = [...]
        let pt_ident: Ident = input.parse()?;
        if pt_ident != "provider_type" {
            return Err(syn::Error::new(pt_ident.span(), "expected `provider_type`"));
        }
        input.parse::<Token![=]>()?;
        let pt_content;
        syn::bracketed!(pt_content in input);
        let provider_type: TokenStream2 = pt_content.parse()?;
        input.parse::<Token![,]>()?;

        // where_clause = [...]
        let wc_ident: Ident = input.parse()?;
        if wc_ident != "where_clause" {
            return Err(syn::Error::new(wc_ident.span(), "expected `where_clause`"));
        }
        input.parse::<Token![=]>()?;
        let wc_content;
        syn::bracketed!(wc_content in input);
        let where_clause: TokenStream2 = wc_content.parse()?;
        input.parse::<Token![,]>()?;

        // fields_prefix = [...]
        let fp_ident: Ident = input.parse()?;
        if fp_ident != "fields_prefix" {
            return Err(syn::Error::new(fp_ident.span(), "expected `fields_prefix`"));
        }
        input.parse::<Token![=]>()?;
        let fp_content;
        syn::bracketed!(fp_content in input);
        let fields_prefix: TokenStream2 = fp_content.parse()?;

        // optional trailing comma
        let _ = input.parse::<Token![,]>();

        Ok(FinalizeInput {
            exports,
            provider_generics,
            provider_type,
            where_clause,
            fields_prefix,
        })
    }
}

pub(crate) fn handle_finalize_imports(item: TokenStream) -> syn::Result<TokenStream> {
    let input = syn::parse::<FinalizeInput>(item)?;

    if input.exports.is_empty() {
        return Ok(TokenStream::new());
    }

    let provider_generics = &input.provider_generics;
    let provider_type = &input.provider_type;
    let where_clause = &input.where_clause;
    let fields_prefix = &input.fields_prefix;

    // Group exports by type (string representation for grouping)
    let exports_with_index: Vec<(usize, &ExportEntry)> = input.exports.iter().enumerate().collect();

    let grouped = group_by(exports_with_index.iter(), |(_, e)| e.ty.to_string());

    let mut outputs = Vec::new();

    for entries in grouped.values() {
        let (_, last_entry) = entries.last().unwrap();
        let ty = &last_entry.ty;
        let field = &last_entry.field;

        // Generate Provider<T> impl - last field wins
        let provider_impl = quote! {
            impl<'prov, #provider_generics> nject::Provider<'prov, #ty> for #provider_type
                where #where_clause
            {
                #[inline]
                fn provide(&'prov self) -> #ty {
                    nject::RefInjectable::<#ty, Self>::inject(&self.#fields_prefix #field, self)
                }
            }
        };
        outputs.push(provider_impl);

        // Generate Iterable<T> if there are multiple exports of this type
        if entries.len() > 1 {
            // We need to track per-field sub-indexes for RefIterable.
            // Group entries by field to compute sub-indexes correctly.
            let iter_match_arms_with_subindex: Vec<TokenStream2> = {
                let mut field_counters: std::collections::HashMap<String, usize> =
                    std::collections::HashMap::new();
                entries
                    .iter()
                    .enumerate()
                    .map(|(iter_index, (_, entry))| {
                        let entry_field = &entry.field;
                        let entry_ty = &entry.ty;
                        let field_key = entry_field.to_string();
                        let sub_index = *field_counters.get(&field_key).unwrap_or(&0);
                        field_counters.insert(field_key, sub_index + 1);
                        let sub_index_lit = syn::LitInt::new(&sub_index.to_string(), proc_macro2::Span::call_site());
                        quote! {
                            #iter_index => nject::RefIterable::<#entry_ty, #provider_type>::inject(&self.provider.#fields_prefix #entry_field, self.provider, #sub_index_lit),
                        }
                    })
                    .collect()
            };

            let iterable_impl = quote! {
                impl<'prov, #provider_generics> nject::Iterable<'prov, #ty> for #provider_type
                    where #where_clause
                {
                    #[inline]
                    fn iter(&'prov self) -> impl Iterator<Item = #ty> {
                        struct NjectIterator<'prov, #provider_generics> {
                            provider: &'prov #provider_type,
                            index: usize,
                        }
                        impl<'prov, #provider_generics> Iterator for NjectIterator<'prov, #provider_generics> {
                            type Item = #ty;

                            fn next(&mut self) -> Option<Self::Item> {
                                let result = match self.index {
                                    #(#iter_match_arms_with_subindex)*
                                    _ => {
                                        return None;
                                    }
                                };
                                self.index += 1;
                                Some(result)
                            }
                        }
                        NjectIterator {
                            provider: self,
                            index: 0,
                        }
                    }
                }
            };
            outputs.push(iterable_impl);
        }
    }

    let result = quote! { #(#outputs)* };
    Ok(result.into())
}
