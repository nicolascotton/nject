use crate::core::{collection::group_by, error, DeriveInput, FactoryExpr, FieldFactoryExpr};
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    Expr, Field, GenericParam, Ident, Lifetime, LifetimeParam, PatType, Token, Type,
};

enum ProvideStructInput {
    TypeExpr(Type, Box<Expr>),
    TypeExprFact(Type, Vec<PatType>, Box<Expr>),
}
impl Parse for ProvideStructInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let parsed_type = input.parse()?;
        input.parse::<Token![,]>()?;
        if input.peek(Token![|]) {
            let expr = FactoryExpr::parse(input)?;
            Ok(Self::TypeExprFact(parsed_type, expr.inputs, expr.body))
        } else {
            Ok(Self::TypeExpr(parsed_type, input.parse()?))
        }
    }
}

type ProvideFieldInput = FieldFactoryExpr;

pub(crate) fn handle_provider(
    item: proc_macro::TokenStream,
) -> syn::Result<proc_macro::TokenStream> {
    let input = syn::parse::<DeriveInput>(item)?;
    let ident = &input.ident;
    let fields = input.fields().iter().collect::<Vec<_>>();
    let generic_keys = input.generic_keys();
    let generic_params = input.generic_params();
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
        .filter_map(|(i, f)| {
            f.attrs
                .iter()
                .filter(|a| a.path().is_ident("import"))
                .next_back()
                .map(|_| i)
        })
        .collect::<Vec<_>>();
    let provide_attr_indexes = fields
        .iter()
        .enumerate()
        .filter_map(|(i, f)| {
            let attrs = f
                .attrs
                .iter()
                .filter(|a| a.path().is_ident("provide"))
                .collect::<Vec<_>>();
            if attrs.is_empty() {
                None
            } else {
                Some((i, attrs))
            }
        })
        .collect::<Vec<_>>();
    let provide_input_attr = input
        .attrs
        .iter()
        .filter(|a| a.path().is_ident("provide"))
        .collect::<Vec<_>>();
    let scope_attr = input
        .attrs
        .iter()
        .filter(|a| a.path().is_ident("scope"))
        .collect::<Vec<_>>();

    let fields_path_prefix = quote! {};
    let import_outputs = gen_imports_for_import_attr(
        ident,
        &generic_params,
        &generic_keys,
        &where_predicates,
        &fields_path_prefix,
        &fields,
        &import_attr_indexes,
    );
    let provide_outputs = gen_providers_for_provide_attr_on_fields(
        ident,
        &generic_params,
        &generic_keys,
        &where_predicates,
        &fields_path_prefix,
        &fields,
        &provide_attr_indexes,
    );
    let input_provide_outputs = gen_providers_for_provide_attr_on_struct(
        ident,
        &generic_params,
        &generic_keys,
        &where_predicates,
        &provide_input_attr,
    );
    let scope_output = gen_scope_output(GenScopeOuptutInput {
        visibility: &input.vis,
        ident,
        generic_params: &generic_params,
        generic_keys: &generic_keys,
        where_predicates: &where_predicates,
        fields: &fields,
        import_attr_indexes: &import_attr_indexes,
        provide_attr_indexes: &provide_attr_indexes,
        provide_input_attr: &provide_input_attr,
        scope_input_attr: &scope_attr,
    })?;

    let output = quote! {
        #[derive(nject::ProviderHelperAttr)]
        #input

        impl<'prov, #(#generic_params,)*Njecty> nject::Provider<'prov, Njecty> for #ident<#(#generic_keys),*>
        where Njecty: nject::Injectable<'prov, Njecty, #ident<#(#generic_keys),*>>, #where_predicates
        {
            #[inline]
            fn provide(&'prov self) -> Njecty {
                Njecty::inject(self)
            }
        }

        impl<'prov, #(#generic_params,)*Njecty> nject::Provider<'prov, &'prov dyn nject::Provider<'prov, Njecty>> for #ident<#(#generic_keys),*>
        where Self: nject::Provider<'prov, Njecty>, #where_predicates
        {
            #[inline]
            fn provide(&'prov self) -> &'prov dyn nject::Provider<'prov, Njecty> {
                self
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

        impl<#(#generic_params),*> #ident<#(#generic_keys),*>
        where #where_predicates
        {
            #[inline]
            pub fn iter<'prov, Value>(&'prov self) -> impl Iterator<Item = Value> + use<'prov #(,#generic_keys)*, Value>
            where Self: nject::Iterable<'prov, Value>
            {
                nject::Iterable::<'prov, Value>::iter(self)
            }
        }
        #(#import_outputs)*
        #(#provide_outputs)*
        #(#input_provide_outputs)*

        #scope_output
    };
    Ok(output.into())
}

fn gen_imports_for_import_attr(
    ident: &Ident,
    generic_params: &[&GenericParam],
    generic_keys: &[proc_macro2::TokenStream],
    where_predicates: &proc_macro2::TokenStream,
    fields_path_prefix: &proc_macro2::TokenStream,
    fields: &[&syn::Field],
    import_attr_indexes: &[usize],
) -> Vec<proc_macro2::TokenStream> {
    let imported_modules = import_attr_indexes
        .iter()
        .map(|i| {
            let field = fields[*i];
            let ty = &field.ty;
            let import_key = super::module::models::ModuleKey::from(ty);
            let import = super::module::repository::get(&import_key);
            let index = syn::Index::from(*i);
            let field_key = match &field.ident {
                Some(i) => quote! { #i },
                None => quote! { #index },
            };
            (import, field_key, field)
        })
        .collect::<Vec<_>>();
    let exported_types = imported_modules.iter().flat_map(|(module, field_key, _)| {
        module
            .as_ref()
            .map(|m| {
                m.exported_types()
                    .iter()
                    .map(|t| (m.to_owned(), t.to_owned()))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default()
            .iter()
            .map(|(m, t)| (m.to_owned(), t.to_owned(), field_key))
            .collect::<Vec<_>>()
    });
    let exported_types = group_by(exported_types, |(_, t, _)| quote! { #t }.to_string());
    let import_iter_outputs = exported_types.values().map(|types| {
        let (_, ty, _) = types.first().unwrap();
        let types_by_module = group_by(types.iter().enumerate(), |(_, (m, _, _))| {
            m.key().unwrap()
        });
        let iter_match_outputs = types_by_module.iter().flat_map(|(_, types_for_module)| {
            types_for_module.iter().enumerate().map(|(mod_index, (index, (_, ty, field_key)))| {
                quote! { #index => nject::RefIterable::<#ty, #ident<#(#generic_keys),*>>::inject(&self.provider.#fields_path_prefix #field_key, self.provider, #mod_index), }
            })
        });
        quote!{

            impl<'prov, #(#generic_params),*> nject::Iterable<'prov, #ty> for #ident<#(#generic_keys),*>
                where #where_predicates
            {
                #[inline]
                fn iter(&'prov self) -> impl Iterator<Item = #ty> {
                    struct NjectIterator<'prov, #(#generic_params),*> {
                        provider: &'prov #ident<#(#generic_keys),*>,
                        index: usize,
                    }
                    impl<'prov, #(#generic_params),*> Iterator for NjectIterator<'prov, #(#generic_keys),*> {
                        type Item = #ty;

                        fn next(&mut self) -> Option<Self::Item> {
                            let result = match self.index {
                                #( #iter_match_outputs )*
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
    });
    let import_prov_outputs = exported_types.values().map(|types| {
        let (_, ty, field_key) = types.last().unwrap();
        quote!{

            impl<'prov, #(#generic_params),*> nject::Provider<'prov, #ty> for #ident<#(#generic_keys),*>
                where #where_predicates
            {
                #[inline]
                fn provide(&'prov self) -> #ty {
                    nject::RefInjectable::<#ty, Self>::inject(&self.#fields_path_prefix #field_key, self)
                }
            }
        }
    });
    let import_impl_outputs = imported_modules.iter().map(|(_, field_key, field)| {
        let ty = &field.ty;
        let ty_output = match ty {
            Type::Reference(r) => {
                let inner_ty = &r.elem;
                quote! { #inner_ty }
            }
            _ => quote! { #ty },
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
    import_impl_outputs
        .chain(import_prov_outputs)
        .chain(import_iter_outputs)
        .collect()
}

fn gen_providers_for_provide_attr_on_fields(
    ident: &Ident,
    generic_params: &[&GenericParam],
    generic_keys: &[proc_macro2::TokenStream],
    where_predicates: &proc_macro2::TokenStream,
    fields_path_prefix: &proc_macro2::TokenStream,
    fields: &[&syn::Field],
    provide_attr_indexes: &[(usize, Vec<&syn::Attribute>)],
) -> Vec<proc_macro2::TokenStream> {
    let provide_outputs = provide_attr_indexes.iter().map(|(i, attrs)| {
        let field = fields[*i];
        let (key_prefix, ref_prefix) = if let Type::Reference(r) = &field.ty {
            let lifetime = &r.lifetime;
            (quote! {}, quote! { &#lifetime })
        } else {
            (quote! { & }, quote! { &'prov })
        };
        let inputs = attrs.iter().map(|a| match a.meta {
            syn::Meta::Path(_) => ProvideFieldInput::Type(field.ty.to_owned()),
            _ => a.parse_args::<ProvideFieldInput>().unwrap()
        });
        let index = syn::Index::from(*i);
        let field_key = match &field.ident {
            Some(i) => quote! { #i },
            None => quote! { #index },
        };
        let outputs = inputs.map(|input| {
            let ty = match &input {
                ProvideFieldInput::None =>  match &field.ty {
                    Type::Reference(r) => {
                        let inner_ty = &r.elem;
                        quote! { #ref_prefix #inner_ty }
                    },
                    _ => {
                        let ty = &field.ty;
                        quote! { #ref_prefix #ty }
                    },
                },
                ProvideFieldInput::Type(t) => match t {
                    Type::Reference(r) => {
                        let inner_ty = &r.elem;
                        quote! { #ref_prefix #inner_ty }
                    },
                    _ => quote! { #ref_prefix #t },
                },
                ProvideFieldInput::TypeExpr(t, _, _) => quote! { #t },
            };
            let body = match &input {
                ProvideFieldInput::TypeExpr(_, i, e) => {
                    let ref_prefix = match & field.ty {
                        Type::Reference(_) => quote!{},
                        _ => quote!{&},
                    };
                    quote!{
                        let #i = #ref_prefix self.#fields_path_prefix #field_key;
                        #e
                    }
                },
                _ => quote!{ #key_prefix self.#fields_path_prefix #field_key }
            };

            quote! {

                impl<'prov, #(#generic_params),*> nject::Provider<'prov, #ty> for #ident<#(#generic_keys),*>
                    where #where_predicates
                {
                    #[inline]
                    fn provide(&'prov self) -> #ty {
                       #body
                    }
                }
            }
        });

        quote! {
            #(#outputs)*
        }
    });
    provide_outputs.collect()
}

pub(crate) fn gen_providers_for_provide_attr_on_struct(
    ident: &Ident,
    generic_params: &[&GenericParam],
    generic_keys: &[proc_macro2::TokenStream],
    where_predicates: &proc_macro2::TokenStream,
    provide_input_attr: &[&syn::Attribute],
) -> Vec<proc_macro2::TokenStream> {
    let input_provide_outputs = provide_input_attr
        .iter()
        .map(|a| a.parse_args::<ProvideStructInput>().unwrap())
        .map(|t| {
            let (ty, inputs, value) = match t {
                ProvideStructInput::TypeExpr(t, v) => (t, vec![], v),
                ProvideStructInput::TypeExprFact(t, i, v) =>(t, i, v),
            };
            quote!{

                impl<'prov, #(#generic_params),*> nject::Provider<'prov, #ty> for #ident<#(#generic_keys),*>
                    where #where_predicates
                {
                    #[inline]
                    fn provide(&'prov self) -> #ty {
                        #(let #inputs = self.provide();)*
                        #value
                    }
                }
            }
        });
    input_provide_outputs.collect()
}

struct GenScopeOuptutInput<'a> {
    visibility: &'a syn::Visibility,
    ident: &'a Ident,
    generic_params: &'a [&'a GenericParam],
    generic_keys: &'a [proc_macro2::TokenStream],
    where_predicates: &'a proc_macro2::TokenStream,
    fields: &'a [&'a syn::Field],
    import_attr_indexes: &'a [usize],
    provide_attr_indexes: &'a [(usize, Vec<&'a syn::Attribute>)],
    provide_input_attr: &'a [&'a syn::Attribute],
    scope_input_attr: &'a [&'a syn::Attribute],
}

fn gen_scope_output(
    GenScopeOuptutInput {
        visibility,
        ident,
        generic_params,
        generic_keys,
        where_predicates,
        fields,
        import_attr_indexes,
        provide_attr_indexes,
        provide_input_attr,
        scope_input_attr,
    }: GenScopeOuptutInput<'_>,
) -> syn::Result<proc_macro2::TokenStream> {
    if scope_input_attr.is_empty() {
        return Ok(proc_macro2::TokenStream::new());
    }
    let scope_lifetime = &GenericParam::Lifetime(LifetimeParam {
        lifetime: Lifetime::new("'scope", Span::call_site()),
        attrs: vec![],
        colon_token: None,
        bounds: Punctuated::default(),
    });
    let mut scope_generic_params = vec![scope_lifetime];
    scope_generic_params.extend_from_slice(generic_params);
    let mut scope_generic_keys = vec![quote! {'scope}];
    scope_generic_keys.extend_from_slice(generic_keys);

    let scope_fields = scope_input_attr
        .iter()
        .map(|a| {
            a.parse_args_with(parse_scope_field).map_err(|e| {
                error::combine(syn::Error::new(a.span(), "Unable to parse scope field."), e)
            })
        })
        .collect::<syn::Result<Vec<_>>>()?;
    let grouped_fields = group_by(scope_fields.iter(), |k| {
        k.ident.as_ref().map(|i| i.to_string())
    });
    let scope_outputs = grouped_fields.iter().map(|(scope_name, scope_fields)|{
        let scope_ident = match scope_name {
            Some(n) => format_ident!("{}{}Scope", ident, snake_to_pascal(n)),
            None => format_ident!("{ident}Scope"),
        };
        let scope_fn_ident = match scope_name {
            Some(n) => format_ident!("{n}_scope"),
            None => format_ident!("scope"),
        };
    let arg_scope_fields =  scope_fields.iter()
        .map(|f| f.attrs.iter().filter(|a| match &a.meta {
            syn::Meta::Path(p) => p.is_ident("arg"),
            _ => false,
        }).next_back().is_some()).collect::<Vec<_>>();
        let scope_field_outputs = scope_fields.iter().map(|f| {
            let mut f = f.to_owned().to_owned();
            f.ident = None;
            quote!{#[provide] #f}
        });
        let root_path = syn::Index::from(scope_fields.len());
        let fields_path_prefix = quote!{#root_path.};
        let import_outputs = gen_imports_for_import_attr(&scope_ident, &scope_generic_params, &scope_generic_keys, where_predicates, &fields_path_prefix, fields, import_attr_indexes);
        let provide_outputs = gen_providers_for_provide_attr_on_fields(&scope_ident, &scope_generic_params, &scope_generic_keys, where_predicates, &fields_path_prefix, fields, provide_attr_indexes);
        let input_provide_outputs = gen_providers_for_provide_attr_on_struct(&scope_ident, &scope_generic_params, &scope_generic_keys, where_predicates, provide_input_attr);
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
                pub fn #scope_fn_ident<'scope>(&'scope self, #(#scope_args),*) -> #scope_ident<#(#scope_generic_keys),*>
                {
                    #scope_ident(#(#scope_field_provides,)* self)
                }
            }

            #[provider]
            #[injectable]
            #[derive(nject::ScopeHelperAttr)]
            #visibility struct #scope_ident<'scope, #(#generic_params),*>(#(#scope_field_outputs,)* &'scope #ident<#(#generic_keys),*>)
                where #where_predicates;

            #(#import_outputs)*
            #(#provide_outputs)*
            #(#input_provide_outputs)*
        }
    });
    Ok(quote! {#(#scope_outputs)*})
}

// Converts a snake case string to a pascal case string
fn snake_to_pascal(snake: &str) -> String {
    let words = snake.split('_');
    words
        .map(|word| {
            let mut chars = word.chars();
            let first = chars.next();
            match first {
                Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect()
}

fn parse_scope_field(input: ParseStream) -> syn::Result<Field> {
    if input.peek(Ident) && input.peek2(Token![:]) {
        let ident = Ident::parse(input)?;
        let _token: Token![:] = input.parse()?;
        let mut field = Field::parse_unnamed(input)?;
        field.ident = Some(ident);
        Ok(field)
    } else {
        Field::parse_unnamed(input)
    }
}
