#![allow(clippy::needless_doctest_main)]
#![doc = include_str!("../README.md")]
mod core;
mod inject;
mod injectable;
mod module;
mod provider;
use inject::handle_inject;
use injectable::handle_injectable;
use module::handle_module;
use proc_macro::TokenStream;
use provider::handle_provider;

/// For internal purposes only. Should not be used.
#[proc_macro_derive(InjectableHelperAttr, attributes(inject))]
pub fn injectable_helper_attr(_item: TokenStream) -> TokenStream {
    TokenStream::new()
}

/// For internal purposes only. Should not be used.
#[proc_macro_derive(ModuleHelperAttr, attributes(export))]
pub fn module_helper_attr(_item: TokenStream) -> TokenStream {
    TokenStream::new()
}

/// For internal purposes only. Should not be used.
#[proc_macro_derive(ProviderHelperAttr, attributes(import, provide, scope))]
pub fn provider_helper_attr(_item: TokenStream) -> TokenStream {
    TokenStream::new()
}

/// For internal purposes only. Should not be used.
#[proc_macro_derive(ScopeHelperAttr, attributes(arg))]
pub fn scope_helper_attr(_item: TokenStream) -> TokenStream {
    TokenStream::new()
}

/// Mark a struct as injectable.
/// ```rust
/// use nject::{injectable, provider};
///
/// #[injectable]
/// struct Facade;
///
/// #[provider]
/// struct Provider;
///
/// let facade: Facade = Provider.provide();
/// ```
#[proc_macro_attribute]
pub fn injectable(_attr: TokenStream, item: TokenStream) -> TokenStream {
    handle_injectable(item).unwrap_or_else(|e| e.to_compile_error().into())
}

/// Use the given value to inject.
/// ```rust
/// use nject::{inject, injectable, provider};
///
/// #[inject(Self { value: 42 })]
/// struct DepOne {
///     value: i32,
/// }
///
/// #[inject(|injectable_dep: DepOne| Self(12, injectable_dep))]
/// struct DepTwo(i32, DepOne);
///
/// #[injectable]
/// struct Facade(DepOne, DepTwo, #[inject(123)] i32);
///
/// #[provider]
/// struct Provider;
///
/// let facade: Facade = Provider.provide();
/// ```
#[proc_macro_attribute]
pub fn inject(attr: TokenStream, item: TokenStream) -> TokenStream {
    handle_inject(item, attr).unwrap_or_else(|e| e.to_compile_error().into())
}

/// Provide a value for a specific type.
/// ```rust
/// use nject::{injectable, provider};
///
/// struct Dependency {
///     value: i32,
/// }
///
/// struct SharedDependency {
///     value: i32,
/// }
///
/// #[injectable]
/// struct Facade<'a>(Dependency, &'a SharedDependency);
///
/// #[provider]
/// #[provide(Dependency, Dependency { value: 123 })]
/// struct Provider {
///     #[provide]
///     shared: SharedDependency
/// }
///
/// let provider = Provider { shared: SharedDependency { value: 456 } };
/// let dependency: Dependency = provider.provide();
/// let facade: Facade = provider.provide();
/// ```
#[proc_macro_attribute]
pub fn provider(_attr: TokenStream, item: TokenStream) -> TokenStream {
    handle_provider(item).unwrap_or_else(|e| e.to_compile_error().into())
}

/// Declare a module to export internal types.
/// ```rust
/// use nject::{injectable, provider};
///
/// mod sub {
///     use nject::{injectable, module};
///     use std::rc::Rc;
///
///     #[injectable]
///     struct InternalType(#[inject(123)] i32); // Not visible outside of module.
///
///     #[injectable]
///     pub struct Facade<'a> {
///         hidden: &'a InternalType,
///         public: Rc<i32>,
///     }
///
///     #[injectable]
///     // The absolute public path to access the module.
///     // If no path is given, the struct name will be used and must be unique across all modules.
///     // Keywords like `crate` and `Self` will be substituted accordingly.
///     #[module(crate::sub::Self)]
///     // Public type exports must be made on the struct (not the fields).
///     // To prevent name collisions, use absolute paths in types.
///     #[export(std::rc::Rc<i32>, self.public.clone())]
///     pub struct Module {
///         #[export] // Fields exports are for internal types.
///         hidden: InternalType,
///         #[inject(Rc::new(456))]
///         public: Rc<i32>,
///     }
/// }
///
/// #[injectable]
/// #[provider]
/// struct Provider {
///     #[import]
///     // To import module public exports, use the absolute path given in its definition.
///     sub_mod: crate::sub::Module,
/// }
///
/// #[provider]
/// struct InitProvider;
///
/// fn main() {
///     let provider = InitProvider.provide::<Provider>();
///     let facade = provider.provide::<sub::Facade>();
/// }
/// ```
#[proc_macro_attribute]
pub fn module(attr: TokenStream, item: TokenStream) -> TokenStream {
    handle_module(attr, item).unwrap_or_else(|e| e.to_compile_error().into())
}
