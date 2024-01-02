use proc_macro;
use quote::{quote, format_ident};
use syn::{parse_macro_input, DeriveInput, GenericParam, Type, Expr, Token, parse::{ParseStream, Parse}, Ident};

struct TypeExpr(Type, Expr);
impl Parse for TypeExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let parsed_type = input.parse()?;
        input.parse::<Token![,]>()?;
        let parsed_value: Expr = input.parse()?;
        return Ok(TypeExpr(parsed_type, parsed_value));
    }
}

pub(crate) fn handle_provider(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
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
    let generic_params = generic_params.iter().map(|p| quote!{#p}).collect::<Vec<_>>();
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
        .filter_map( |(i, f)| { 
            let attrs = f.attrs.iter().filter(|a| a.path().is_ident("provide")).collect::<Vec<_>>();
            if attrs.is_empty() {
                None
            } else {
                Some((i, attrs))
            }
        })
        .collect::<Vec<_>>();
    let provide_input_attr = input.attrs.iter().filter(|a| a.path().is_ident("provide")).collect::<Vec<_>>();
    let scope_attr = input.attrs.iter().filter(|a| a.path().is_ident("scope")).collect::<Vec<_>>();

    let fields_path_prefix = quote!{};
    let import_outputs = gen_imports_for_import_attr(&ident, &generic_params, &generic_keys, &where_predicates, &fields_path_prefix, &fields, &import_attr_indexes);
    let provide_outputs = gen_providers_for_provide_attr(&ident, &generic_params, &generic_keys, &where_predicates, &fields_path_prefix, &fields, &provide_attr_indexes);
    let input_provide_outputs = gen_providers_for_provide_input_attributes(&ident, &generic_params, &generic_keys, &where_predicates, &provide_input_attr);
    let scope_output = gen_scope_output(&input.vis, &ident, &generic_params, &generic_keys, &where_predicates, &fields, &import_attr_indexes, &provide_attr_indexes, &provide_input_attr, &scope_attr);

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

        #scope_output
    };
    output.into()
}

fn gen_imports_for_import_attr(
    ident: & Ident, 
    generic_params: &[proc_macro2::TokenStream], 
    generic_keys: &[proc_macro2::TokenStream], 
    where_predicates: &proc_macro2::TokenStream,
    fields_path_prefix: &proc_macro2::TokenStream,
    fields: &[&syn::Field],
    import_attr_indexes: &[usize]
 ) -> Vec<proc_macro2::TokenStream> {
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
                    &self.#fields_path_prefix #field_key
                }
            }
        }
    });
    import_outputs.collect()
}

fn gen_providers_for_provide_attr(
    ident: &Ident, 
    generic_params: &[proc_macro2::TokenStream], 
    generic_keys: &[proc_macro2::TokenStream], 
    where_predicates: &proc_macro2::TokenStream,
    fields_path_prefix: &proc_macro2::TokenStream,
    fields: &[&syn::Field], 
    provide_attr_indexes: &[(usize, Vec<&syn::Attribute>)]
) -> Vec<proc_macro2::TokenStream> {
    let provide_outputs = provide_attr_indexes.iter().map(|(i, attrs)| {
        let field = fields[*i];
        let (key_prefix, ref_prefix) = if let Type::Reference(r) = &field.ty {
            let lifetime = &r.lifetime;
            (quote! {}, quote! { &#lifetime })
        } else {
            (quote! { & }, quote! { &'prov })
        };
        let attr_types = attrs.iter().map(|a| match a.meta {
            syn::Meta::Path(_) => field.ty.to_owned(),
            _ => a.parse_args::<Type>().unwrap()
        });
        let ty_outputs = attr_types.map(|ty| match ty {
            Type::Reference(r) => {
                let inner_ty = &r.elem;
                quote! { #ref_prefix #inner_ty }
            },
            _ => quote! { #ref_prefix #ty },
        });
        let index = syn::Index::from(*i);
        let field_key = match &field.ident {
            Some(i) => quote! { #i },
            None => quote! { #index },
        };
        let outputs = ty_outputs.map(|ty_output| quote! {

            impl<'prov#(,#generic_params)*> nject::Provider<'prov, #ty_output> for #ident<#(#generic_keys),*>
                where #where_predicates
            {
                #[inline]
                fn provide(&'prov self) -> #ty_output {
                    #key_prefix self.#fields_path_prefix #field_key
                }
            }
        });

        quote! {
            #(#outputs)*
        }
    });
    provide_outputs.collect()
}

fn gen_providers_for_provide_input_attributes<'a>(
    ident: &Ident, 
    generic_params: &[proc_macro2::TokenStream], 
    generic_keys: &[proc_macro2::TokenStream], 
    where_predicates: &proc_macro2::TokenStream,
    provide_input_attr: &[&syn::Attribute],
) -> Vec<proc_macro2::TokenStream> {
    let input_provide_outputs = provide_input_attr
        .iter()
        .map(|a| a.parse_args::<TypeExpr>().unwrap())
        .map(|t| {
            let ty = t.0;
            let value= t.1;
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
    input_provide_outputs.collect()
}

fn gen_scope_output(
    visibility: &syn::Visibility,
    ident: &Ident, 
    generic_params: &[proc_macro2::TokenStream], 
    generic_keys: &[proc_macro2::TokenStream], 
    where_predicates: &proc_macro2::TokenStream,
    fields: &[&syn::Field], 
    import_attr_indexes: &[usize],
    provide_attr_indexes: &[(usize, Vec<&syn::Attribute>)],
    provide_input_attr: &[&syn::Attribute],
    scope_input_attr: &[&syn::Attribute]
) -> proc_macro2::TokenStream {
    if scope_input_attr.is_empty() {
        return proc_macro2::TokenStream::new();
    }
    let scope_ident = format_ident!("{ident}Scope");
    let mut scope_generic_params = vec![quote!{'scope}];
    scope_generic_params.extend_from_slice(generic_params);
    let mut scope_generic_keys = vec![quote!{'scope}];
    scope_generic_keys.extend_from_slice(generic_keys);

    let scope_fields = scope_input_attr
        .iter()
        .map(|a| a.parse_args_with(syn::Field::parse_unnamed).unwrap())
        .collect::<Vec<_>>();
    let arg_scope_fields =  scope_fields.iter()
        .map(|f| match f.attrs.iter().filter(|a| match &a.meta {
            syn::Meta::Path(p) => p.is_ident("provide"), 
            _ => false,
        }).last() {
            Some(_) => true,
            None => false,
        }).collect::<Vec<_>>();
    let scope_field_outputs = scope_fields.iter()
        .enumerate()
        .map(|(i, f)| match arg_scope_fields[i] {
           true => quote!{#f},
           false => quote!{#[provide] #f},
        });
    let root_path = syn::Index::from(scope_fields.len());
    let fields_path_prefix = quote!{#root_path.};
    let import_outputs = gen_imports_for_import_attr(&scope_ident, &scope_generic_params, &scope_generic_keys, &where_predicates, &fields_path_prefix, &fields, &import_attr_indexes);
    let provide_outputs = gen_providers_for_provide_attr(&scope_ident, &scope_generic_params, &scope_generic_keys, &where_predicates, &fields_path_prefix, &fields, &provide_attr_indexes);
    let input_provide_outputs = gen_providers_for_provide_input_attributes(&scope_ident, &scope_generic_params, &scope_generic_keys, &where_predicates, &provide_input_attr);
    let scope_field_provides = scope_fields.iter()
        .enumerate()
        .map(|(i, _)| match arg_scope_fields[i] {
            true => {
                let ident = format_ident!("v{i}");
                quote!{#ident}
            },
            false => quote!{self.provide()} 
        });
    let scope_args = scope_fields.iter()
        .enumerate()
        .filter_map(|(i, f)| match arg_scope_fields[i] {
            true => {
                let ident = format_ident!("v{i}");
                let ty = &f.ty;
                Some(quote!{#ident: #ty})
            },
            false => None
        });

    quote!{
        impl<#(#generic_params),*> #ident<#(#generic_keys),*>
            where #where_predicates
        {
            #[inline]
            pub fn scope<'scope>(&'scope self #(,#scope_args)*) -> #scope_ident<#(#scope_generic_keys),*>
            {
                #scope_ident(#(#scope_field_provides,)* self)
            }
        }

        #[provider]
        #visibility struct #scope_ident<'scope, #(#generic_params),*>(#(#scope_field_outputs,)* &'scope #ident<#(#generic_keys),*>)
            where #where_predicates;

        #(#import_outputs)*
        #(#provide_outputs)*
        #(#input_provide_outputs)*
    }
}