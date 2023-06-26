use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, GenericParam, Type, Expr, Token, parse::{ParseStream, Parse}};

struct TypeExpr(Type, Expr);
impl Parse for TypeExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let parsed_type = input.parse()?;
        input.parse::<Token![,]>()?;
        let parsed_value: Expr = input.parse()?;
        return Ok(TypeExpr(parsed_type, parsed_value));
    }
}

pub(crate) fn handle_provider(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let ident = &input.ident;
    let fields = match &input.data {
        syn::Data::Struct(d) => d.fields.iter().collect::<Vec<_>>(),
        _ => panic!("Unsupported type. Macro should be used on a struct"),
    };
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
    let import_attr_indexes = fields
        .iter()
        .enumerate()
        .filter_map(
            |(i, f)| match f.attrs.iter().filter(|a| a.path().is_ident("import")).last() {
                Some(_) => Some(i),
                None => None,
            },
        )
        .collect::<Vec<_>>();
    let provide_attr_indexes = fields
        .iter()
        .enumerate()
        .filter_map(
            |(i, f)| match f.attrs.iter().filter(|a| a.path().is_ident("provide")).last() {
                Some(_) => Some(i),
                None => None,
            },
        )
        .collect::<Vec<_>>();
    let provide_input_attr = input.attrs.iter().filter(|a| a.path().is_ident("provide"));
    let import_outputs = import_attr_indexes.iter().map(|i| {
        let field = fields[*i];
        let ty = &field.ty;
        let ty_output = match ty {
            Type::Reference(r) => {
                let inner_ty = &r.elem;
                quote! { #inner_ty }
            }
            _ => quote! { #ty },
        };
        let index = syn::Index::from(*i);
        let field_key = match &field.ident {
            Some(i) => quote! { #i },
            None => quote! { #index },
        };

        quote! {

            impl<#(#generic_params),*> nject::Import<#ty_output> for #ident<#(#generic_keys),*>
                where #where_predicates
            {
                #[inline]
                fn reference(&self) -> & #ty_output {
                    &self.#field_key
                }
            }
        }
    });
    let provide_outputs = provide_attr_indexes.iter().map(|i| {
        let field = fields[*i];
        let ty = &field.ty;
        let ty_output = match ty {
            Type::Reference(r) => {
                let inner_ty = &r.elem;
                quote! { #inner_ty }
            }
            _ => quote! { #ty },
        };
        let index = syn::Index::from(*i);
        let field_key = match &field.ident {
            Some(i) => quote! { #i },
            None => quote! { #index },
        };

        quote! {

            impl<'prov#(,#generic_params)*> nject::Provider<'prov, &'prov #ty_output> for #ident<#(#generic_keys),*>
                where #where_predicates
            {
                #[inline]
                fn provide(&'prov self) -> &'prov #ty_output {
                    &self.#field_key
                }
            }
        }
    });

    let input_provide_outputs =
        provide_input_attr.map(|a| a.parse_args::<TypeExpr>().unwrap())
        .map(|t| {
            let ty = t.0;
            let value=t.1;
            quote!{ 

                impl<'prov, #(#generic_params),*> nject::Provider<'prov, #ty> for #ident<#(#generic_keys),*>
                    where #where_predicates
                {
                    #[inline]
                    fn provide(&'prov self) -> #ty {
                        #value
                    }
                }
            }
        });

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
        #(#import_outputs)*
        #(#provide_outputs)*
        #(#input_provide_outputs)*
    };
    output.into()
}
