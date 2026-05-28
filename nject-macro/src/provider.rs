use crate::core::{DeriveInput, FactoryExpr, FieldFactoryExpr, collection::group_by, error};
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::{
    Expr, Field, GenericParam, Ident, Lifetime, LifetimeParam, PatType, Token, Type,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
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
                .rfind(|a| a.path().is_ident("import"))
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
    let imported_modules: Vec<_> = import_attr_indexes
        .iter()
        .map(|i| {
            let field = fields[*i];
            let ty = &field.ty;
            let index = syn::Index::from(*i);
            let field_key = match &field.ident {
                Some(i) => quote! { #i },
                None => quote! { #index },
            };
            (field_key, field, ty)
        })
        .collect();

    // Generate Import<Module> impls
    let import_impl_outputs: Vec<proc_macro2::TokenStream> = imported_modules
        .iter()
        .map(|(field_key, _field, ty)| {
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
        })
        .collect();

    // Generate chain invocation for the macro-based system
    let chain_output = gen_chain_invocation(
        ident,
        generic_params,
        generic_keys,
        where_predicates,
        fields_path_prefix,
        fields,
        import_attr_indexes,
    );

    let mut outputs = import_impl_outputs;
    if let Some(chain) = chain_output {
        outputs.push(chain);
    }
    outputs
}

/// Generate the chain invocation that starts the module macro chain.
/// This replaces the disk-based lookup system.
fn gen_chain_invocation(
    ident: &Ident,
    generic_params: &[&GenericParam],
    generic_keys: &[proc_macro2::TokenStream],
    where_predicates: &proc_macro2::TokenStream,
    fields_path_prefix: &proc_macro2::TokenStream,
    fields: &[&syn::Field],
    import_attr_indexes: &[usize],
) -> Option<proc_macro2::TokenStream> {
    if import_attr_indexes.is_empty() {
        return None;
    }

    // Build the list of module info for the chain
    let module_infos: Vec<_> = import_attr_indexes
        .iter()
        .map(|i| {
            let field = fields[*i];
            let ty = &field.ty;
            let index = syn::Index::from(*i);
            let field_key = match &field.ident {
                Some(ident) => quote! { #ident },
                None => quote! { #index },
            };

            // Extract the module path and compute the macro path
            let (macro_path, module_args) = match extract_module_macro_path(ty) {
                Ok(result) => result,
                Err(e) => (e.to_compile_error(), quote! {}),
            };

            (macro_path, field_key, module_args)
        })
        .collect();

    if module_infos.is_empty() {
        return None;
    }

    // Build the `next` list (all modules except the first)
    let next_entries: Vec<proc_macro2::TokenStream> = module_infos[1..]
        .iter()
        .map(|(macro_path, field_key, module_args)| {
            quote! { (#macro_path, [#field_key], [#module_args]) }
        })
        .collect();

    let next_list = quote! { #(#next_entries),* };

    // First module info
    let (first_macro_path, first_field_key, first_module_args) = &module_infos[0];

    // Provider context info
    let provider_generics = quote! { #(#generic_params),* };
    let provider_type = quote! { #ident<#(#generic_keys),*> };

    // Generate the chain start invocation
    Some(quote! {
        #first_macro_path! {
            @nject_collect
            next = [#next_list],
            field = [#first_field_key],
            module_args = [#first_module_args],
            exports = [],
            provider_generics = [#provider_generics],
            provider_type = [#provider_type],
            where_clause = [#where_predicates],
            fields_prefix = [#fields_path_prefix],
        }
    })
}

/// Extract the macro path from a module type.
/// Given a type like `crate::sub::MyModule` or `other_crate::MyModule`,
/// returns the macro invocation path and the generic args (if any).
///
/// Rules:
/// - `crate::...::Module` → same crate → bare `__nject_module_Module`
/// - `self::...::Module` → same crate → bare `__nject_module_Module`
/// - `super::...::Module` → same crate → bare `__nject_module_Module`
/// - `Module` (single segment) → same crate → bare `__nject_module_Module`
/// - `external_crate::...::Module` → external → `external_crate::__nject_module_Module`
///
/// The macro name is based solely on the struct name (last segment).
/// Module struct names must be unique within a single crate.
fn extract_module_macro_path(ty: &Type) -> syn::Result<(proc_macro2::TokenStream, proc_macro2::TokenStream)> {
    let path = match ty {
        Type::Path(p) => &p.path,
        Type::Reference(r) => match &*r.elem {
            Type::Path(p) => &p.path,
            _ => return Err(syn::Error::new_spanned(ty, "Unsupported import type: expected a path type")),
        },
        _ => return Err(syn::Error::new_spanned(ty, "Unsupported import type: expected a path type")),
    };

    let segments: Vec<_> = path.segments.iter().collect();
    if segments.is_empty() {
        return Err(syn::Error::new_spanned(ty, "Empty path for import type"));
    }

    // The last segment is the struct name
    let last_segment = segments.last().unwrap();
    let struct_name = &last_segment.ident;
    let macro_name = format_ident!("__nject_module_{}", struct_name);

    // Extract generic args from the last segment
    let module_args = match &last_segment.arguments {
        syn::PathArguments::None => quote! {},
        syn::PathArguments::AngleBracketed(args) => {
            let args_iter = &args.args;
            quote! { #args_iter }
        }
        syn::PathArguments::Parenthesized(_) => quote! {},
    };

    if segments.len() == 1 {
        // Single segment like `MyModule` - same crate, bare name
        return Ok((quote! { #macro_name }, module_args));
    }

    let first_segment_name = segments[0].ident.to_string();
    let is_same_crate = first_segment_name == "crate"
        || first_segment_name == "self"
        || first_segment_name == "super";

    let macro_path = if is_same_crate {
        // Same crate - use bare name (macro_export puts it at crate root)
        quote! { #macro_name }
    } else {
        // External crate - use crate_name::macro_name
        let crate_prefix = &segments[0].ident;
        quote! { #crate_prefix :: #macro_name }
    };

    Ok((macro_path, module_args))
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
    let scope_outputs = grouped_fields.iter().map(|(scope_name, scope_fields)| {
        let scope_ident = match scope_name {
            Some(n) => format_ident!("{}{}Scope", ident, snake_to_pascal(n)),
            None => format_ident!("{ident}Scope"),
        };
        let scope_fn_ident = match scope_name {
            Some(n) => format_ident!("{n}_scope"),
            None => format_ident!("scope"),
        };
        let arg_scope_fields =  scope_fields.iter()
            .map(|f| f.attrs.iter().rfind(|a| match &a.meta {
                syn::Meta::Path(p) => p.is_ident("arg"),
                _ => false,
            }).is_some()).collect::<Vec<_>>();
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
