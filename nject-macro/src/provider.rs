use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, GenericParam};

pub(crate) fn handle_provider(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let ident = &input.ident;
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
    let output = quote! {
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
    };
    output.into()
}
