use super::repository::models::{ProvidedType, Provider, ProviderKey};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, GenericParam};

pub(crate) fn handle_provider(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let ident = &input.ident;
    let provider_key = ProviderKey::from(ident);
    let generic_params = &input.generics.params.iter().collect::<Vec<&GenericParam>>();
    let generic_keys = &generic_params
        .iter()
        .map(|p| match p {
            GenericParam::Type(t) => {
                let identity = &t.ident;
                quote! { #identity }
            }
            GenericParam::Const(c) => {
                let identity = &c.ident;
                quote! { #identity }
            }
            GenericParam::Lifetime(l) => quote! { #l },
        })
        .collect::<Vec<_>>();
    let where_predicates = match &input.generics.where_clause {
        Some(w) => {
            let predicates = &w.predicates;
            quote! { #predicates }
        }
        None => quote! {},
    };
    let fields = match &input.data {
        syn::Data::Struct(d) => &d.fields,
        _ => panic!("Unsupported type. Macro should be used on a struct"),
    };
    let fields_to_extend = fields.iter().enumerate().filter_map(|(i, f)| {
        match f.attrs.iter().filter(|a| a.path.is_ident("extend")).last() {
            Some(a) => Some((i, f, a)),
            None => None,
        }
    });
    let extends_info = fields_to_extend.flat_map(|(index, field, attr)| {
        let provider_type = &field.ty;
        let index = syn::Index::from(index);
        let field_name = match field.ident.as_ref() {
            Some(i) => quote! { #i },
            None => quote! { #index },
        };
        let current_provider = Provider::from(provider_type);
        let attr_tokens = attr.tokens.to_string();
        let attr = attr_tokens.trim_start_matches('(').trim_end_matches(')');
        if !attr_tokens.is_empty() && attr.len() + 2 != attr_tokens.len() {
            panic!("extend attribute is missing opening and/or closing parentheses.")
        }
        let current_provider_key = if attr.is_empty() {
            ProviderKey::from(provider_type)
        } else {
            ProviderKey(attr.to_owned())
        };
        let extend_provider = super::repository::provider::get(&current_provider_key)
            .expect("Provider must be declared before any extend.");
        let subs_map = extend_provider.generic_substitution_map(&current_provider);
        let output_types = super::repository::provide::get(&current_provider_key);
        output_types
            .iter()
            .map(|p| {
                let extended_output_type = p.to_type_with_generics_substitutions(&subs_map);
                super::repository::provide::add(
                    provider_key.to_owned(),
                    ProvidedType::from(&extended_output_type),
                );
                (extended_output_type, field_name.to_owned())
            })
            .collect::<Vec<_>>()
    });
    let provides_output = extends_info.map(|(output_type, field_name)|{
        quote! {
            impl<'prov, #(#generic_params),*> nject::Provider<'prov, #output_type> for #ident<#(#generic_keys),*>
                where #where_predicates
            {
                #[inline]
                fn provide(&'prov self) -> #output_type {
                    self.#field_name.provide()
                }
            }
        }
    });

    super::repository::provider::add(provider_key.to_owned(), Provider::from(&input));
    super::repository::provide::remove(&provider_key);
    let output = quote! {
        #[derive(nject::ProviderHelperAttr)]
        #input

        impl<'prov, #(#generic_params,)*Njecty> nject::Provider<'prov, Njecty> for #ident<#(#generic_keys),*>
            where Njecty: nject::Injectable<'prov, Njecty, #ident<#(#generic_keys),*>>,#where_predicates
        {
            #[inline]
            fn provide(&'prov self) -> Njecty {
                Njecty::inject(self)
            }
        }

        impl<#(#generic_params),*> #ident<#(#generic_keys),*>
            where #where_predicates
        {
            #[inline]
            pub fn provide<'prov, Njecty>(&'prov self) -> Njecty
                where Self: nject::Provider<'prov, Njecty>
            {
                <Self as nject::Provider<'prov, Njecty>>::provide(self)
            }
        }
        #(#provides_output)*
    };
    output.into()
}
