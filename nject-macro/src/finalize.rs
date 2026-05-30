use crate::core::collection::group_by;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    Ident, Token,
};

fn parse_named_bracketed_tokens(input: ParseStream, name: &str) -> syn::Result<TokenStream2> {
    let ident: Ident = input.parse()?;
    if ident != name {
        return Err(syn::Error::new(ident.span(), format!("expected `{name}`")));
    }

    input.parse::<Token![=]>()?;
    let content;
    syn::bracketed!(content in input);
    content.parse()
}

fn parse_named_bracketed_entries<T: Parse>(input: ParseStream, name: &str) -> syn::Result<Vec<T>> {
    let ident: Ident = input.parse()?;
    if ident != name {
        return Err(syn::Error::new(ident.span(), format!("expected `{name}`")));
    }

    input.parse::<Token![=]>()?;
    let content;
    syn::bracketed!(content in input);

    let mut entries = Vec::new();
    while !content.is_empty() {
        entries.push(content.parse()?);
    }
    Ok(entries)
}

/// A single export entry: `{ field = [field_tokens], ty = [type_tokens] }`
struct ExportEntry {
    field: TokenStream2,
    ty: TokenStream2,
}

impl Parse for ExportEntry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        braced!(content in input);

        let field = parse_named_bracketed_tokens(&content, "field")?;
        content.parse::<Token![,]>()?;
        let ty = parse_named_bracketed_tokens(&content, "ty")?;
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
        let exports = parse_named_bracketed_entries(input, "exports")?;
        input.parse::<Token![,]>()?;

        let provider_generics = parse_named_bracketed_tokens(input, "provider_generics")?;
        input.parse::<Token![,]>()?;

        let provider_type = parse_named_bracketed_tokens(input, "provider_type")?;
        input.parse::<Token![,]>()?;

        let where_clause = parse_named_bracketed_tokens(input, "where_clause")?;
        input.parse::<Token![,]>()?;

        let fields_prefix = parse_named_bracketed_tokens(input, "fields_prefix")?;
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

fn gen_provider_impl(
    entry: &ExportEntry,
    provider_generics: &TokenStream2,
    provider_type: &TokenStream2,
    where_clause: &TokenStream2,
    fields_prefix: &TokenStream2,
) -> TokenStream2 {
    let ty = &entry.ty;
    let field = &entry.field;

    quote! {
        impl<'prov, #provider_generics> nject::Provider<'prov, #ty> for #provider_type
            where #where_clause
        {
            #[inline]
            fn provide(&'prov self) -> #ty {
                nject::RefInjectable::<#ty, Self>::inject(&self.#fields_prefix #field, self)
            }
        }
    }
}

fn gen_iter_match_arms(
    entries: &[&(usize, &ExportEntry)],
    provider_type: &TokenStream2,
    fields_prefix: &TokenStream2,
) -> Vec<TokenStream2> {
    let mut field_counters = std::collections::HashMap::new();

    entries
        .iter()
        .enumerate()
        .map(|(iter_index, (_, entry))| {
            let entry_field = &entry.field;
            let entry_ty = &entry.ty;
            let field_key = entry_field.to_string();
            let sub_index = *field_counters.get(&field_key).unwrap_or(&0);
            field_counters.insert(field_key, sub_index + 1);
            let sub_index_lit =
                syn::LitInt::new(&sub_index.to_string(), proc_macro2::Span::call_site());

            quote! {
                #iter_index => nject::RefIterable::<#entry_ty, #provider_type>::inject(&self.provider.#fields_prefix #entry_field, self.provider, #sub_index_lit),
            }
        })
        .collect()
}

fn gen_iterable_impl(
    entries: &[&(usize, &ExportEntry)],
    ty: &TokenStream2,
    provider_generics: &TokenStream2,
    provider_type: &TokenStream2,
    where_clause: &TokenStream2,
    fields_prefix: &TokenStream2,
) -> TokenStream2 {
    let iter_match_arms_with_subindex = gen_iter_match_arms(entries, provider_type, fields_prefix);

    quote! {
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

        // Generate Provider<T> impl - last field wins
        outputs.push(gen_provider_impl(
            last_entry,
            provider_generics,
            provider_type,
            where_clause,
            fields_prefix,
        ));

        // Generate Iterable<T> for all exported types
        outputs.push(gen_iterable_impl(
            entries,
            ty,
            provider_generics,
            provider_type,
            where_clause,
            fields_prefix,
        ));
    }

    let result = quote! { #(#outputs)* };
    Ok(result.into())
}
