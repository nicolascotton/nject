use crate::core::{extract_path_from_type, substitute_in_path, substitute_in_type};
use quote::{quote, ToTokens};
use syn::{Ident, Path, PathSegment, Type};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct ModuleKey(pub(crate) String);

impl From<&Ident> for ModuleKey {
    fn from(value: &Ident) -> Self {
        Self(quote! {#value}.to_string())
    }
}

impl From<&Type> for ModuleKey {
    fn from(value: &Type) -> Self {
        let mut path = extract_path_from_type(value).to_owned();
        Self::from(&mut path)
    }
}

impl From<&mut Path> for ModuleKey {
    fn from(value: &mut Path) -> Self {
        if let Some(ref crate_name) = current_crate_name() {
            substitute_in_path(value, "crate", crate_name);
        }
        Self(value.to_token_stream().to_string())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Module {
    pub(crate) crate_name: Option<String>,
    pub(crate) bin_name: Option<String>,
    pub(crate) path: String,
    pub(crate) exported_types: Vec<String>,
}

impl Module {
    pub fn key(&self) -> Result<ModuleKey, syn::Error> {
        let path_token_stream = self.path.parse().expect("Unable to parse module path.");
        let mut path = syn::parse::<Path>(path_token_stream)?;
        if let Some(ref crate_name) = self.crate_name {
            substitute_in_path(&mut path, "crate", crate_name);
        }
        Ok(ModuleKey(path.to_token_stream().to_string()))
    }

    pub fn exported_types(&self) -> Vec<Type> {
        if self.exported_types.is_empty() {
            return vec![];
        }
        let mut types = self
            .exported_types
            .iter()
            .map(|t| {
                let type_token_stream = t.parse().expect("Unable to parse module exported type.");
                syn::parse::<Type>(type_token_stream)
                    .expect("Unable to parse module exported type.")
            })
            .collect::<Vec<_>>();

        if let Some(module_crate) = &self.crate_name {
            if let Some(_) = &self.bin_name {
                return types;
            } else if let Some(ref crate_name) = current_crate_name() {
                if module_crate == crate_name && current_bin_name().is_none() {
                    return types;
                }
            }
            for ty in &mut types {
                substitute_in_type(ty, "crate", module_crate);
            }
        }

        types
    }
}

impl From<(&Ident, Option<&Path>, &[&Type])> for Module {
    fn from((ident, path, types): (&Ident, Option<&Path>, &[&Type])) -> Self {
        let crate_name = current_crate_name();
        let mut path = match path {
            Some(p) => p.to_owned(),
            None => Path::from(PathSegment {
                arguments: syn::PathArguments::None,
                ident: ident.to_owned(),
            }),
        };
        let ident = ident.to_string();
        substitute_in_path(&mut path, "Self", &ident);
        Self {
            crate_name,
            bin_name: current_bin_name(),
            path: path.to_token_stream().to_string(),
            exported_types: types
                .iter()
                .map(|t| t.to_token_stream().to_string())
                .collect(),
        }
    }
}

/// Name of the current crate.
fn current_crate_name() -> Option<String> {
    match std::env::var("CARGO_CRATE_NAME") {
        Ok(x) => Some(x),
        Err(_) => None,
    }
}

/// Name of the current binary. If it's a bench or test, the name will be `test_bench`
fn current_bin_name() -> Option<String> {
    match std::env::var("CARGO_BIN_NAME") {
        Ok(x) => Some(x),
        Err(_) => match std::env::var("CARGO_TARGET_TMPDIR") {
            Ok(_) => Some(String::from("test_bench")),
            Err(_) => None,
        },
    }
}
