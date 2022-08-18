use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, GenericParam, Ident, Type};

#[proc_macro_attribute]
pub fn injectable(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let ident = &input.ident;
    let fields = match &input.data {
        syn::Data::Struct(d) => &d.fields,
        _ => panic!("Unsupported type. Macro should be used on a struct."),
    };
    let types = fields.iter().map(|f| &f.ty).collect::<Vec<&Type>>();
    let keys = fields
        .iter()
        .map(|f| f.ident.as_ref())
        .filter_map(|i| i)
        .collect::<Vec<&Ident>>();
    let generics = &input.generics;
    let generic_params = &input.generics.params.iter().collect::<Vec<&GenericParam>>();
    let creation_output = match keys.is_empty() && !types.is_empty() {
        true => {
            let items = types.iter().map(|_| quote! { provider.provide() });
            quote! { #ident(#(#items),*) }
        },
        false => quote! { #ident { #(#keys: provider.provide()),* } },
    };
    let output = quote! {
        #input

        impl<#(#generic_params,)*NjectProvider> nject::Injectable<#ident #generics, NjectProvider> for #ident #generics
            where NjectProvider: #(nject::Provider<#types>)+*
        {
            fn inject(provider: &NjectProvider) -> #ident #generics {
                #creation_output
            }
        }
    };
    output.into()
}

#[proc_macro_attribute]
pub fn provider(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let ident = &input.ident;
    let generics = &input.generics;
    let generic_params = &input.generics.params.iter().collect::<Vec<&GenericParam>>();
    let output = quote! {
        use nject::Provider as _;
        #input

        impl<#(#generic_params,)*Njecty> nject::Provider<Njecty> for #ident #generics
            where Njecty: nject::Injectable<Njecty, #ident #generics>
        {
            fn provide(&self) -> Njecty {
                Njecty::inject(self)
            }
        }
    };
    output.into()
}
