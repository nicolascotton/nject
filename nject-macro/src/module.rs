use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, GenericParam, Type};

pub(crate) fn handle_module(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let ident = &input.ident;
    let fields = match &input.data {
        syn::Data::Struct(d) => d.fields.iter().collect::<Vec<_>>(),
        _ => panic!("Unsupported type. Macro should be used on a struct"),
    };
    let export_attr_indexes = fields
        .iter()
        .enumerate()
        .filter_map(|(i, f)| {
            let attrs = f
                .attrs
                .iter()
                .filter(|a| a.path().is_ident("export"))
                .collect::<Vec<_>>();
            if attrs.is_empty() {
                None
            } else {
                Some((i, attrs))
            }
        })
        .collect::<Vec<_>>();
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
    let lifetime_keys = &generic_params
        .iter()
        .filter_map(|p| match p {
            GenericParam::Lifetime(l) => Some(quote! { #l }),
            _ => None,
        })
        .collect::<Vec<_>>();
    let prov_lifetimes = match lifetime_keys.len() > 0 {
        true => quote! { 'prov: #(#lifetime_keys)+*, },
        false => quote! {},
    };

    let where_predicates = match &input.generics.where_clause {
        Some(w) => {
            let predicates = &w.predicates;
            quote! { #predicates }
        }
        None => quote! {},
    };

    let export_outputs = export_attr_indexes.iter().map(|(i, attrs)| {
        let field = fields[*i];
        let ref_prefix = if let Type::Reference(r) = &field.ty {
            let lifetime = &r.lifetime;
             quote! { &#lifetime }
        } else {
            quote! { &'prov }
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
    		Some(i) => quote!{ #i },
    		None => quote!{ #index },
    	};
        let outputs = ty_outputs.map(|ty_output| quote! {
    		impl<'prov, #(#generic_params,)*NjectProvider> nject::Injectable<'prov, #ty_output, NjectProvider> for #ty_output
    			where
    				#prov_lifetimes
    				NjectProvider: nject::Import<#ident<#(#generic_keys),*>>,#where_predicates
    		{
    			#[inline]
    			fn inject(provider: &'prov NjectProvider) -> #ty_output {
        			&provider.reference().#field_key
    			}
    		}
        });
        quote!{
            #(#outputs)*
        }
    });

    let output = quote! {
        #[derive(nject::ModuleHelperAttr)]
        #input
        #(#export_outputs)*
    };
    output.into()
}
