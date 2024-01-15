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

/// Attribute to mark a struct as injectable.
/// ```rust
/// use nject::{injectable, provider};
///
/// #[injectable]
/// struct Facade;
///
/// #[provider]
/// struct Provider;
///
/// fn main() {
///     let _facade: Facade = Provider.provide();
/// }
/// ```
#[proc_macro_attribute]
pub fn injectable(_attr: TokenStream, item: TokenStream) -> TokenStream {
    handle_injectable(item)
}

/// Attribute to specify a desired injected value.
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
/// fn main() {
///     let _facade: Facade = Provider.provide();
/// }
/// ```
#[proc_macro_attribute]
pub fn inject(attr: TokenStream, item: TokenStream) -> TokenStream {
    handle_inject(item, attr)
}

/// Attribute to provide a given instance for a specific type.
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
/// fn main() {
///     let provider = Provider { shared: SharedDependency { value: 456 } };
///     let _dependency: Dependency = provider.provide();
///     let _facade: Facade = provider.provide();
/// }
/// ```
#[proc_macro_attribute]
pub fn provider(_attr: TokenStream, item: TokenStream) -> TokenStream {
    handle_provider(item)
}

/// Declare a module to export internal types.
/// ```rust
/// use nject::{injectable, provider};
///
/// mod sub {
///     use nject::{injectable, module};
///
///     #[injectable]
///     struct InternalType(#[inject(123)] i32); // Not visible outside of module.
///
///     #[injectable]
///     pub struct Facade<'a> {
///         hidden: &'a InternalType
///     }
///
///     #[injectable]
///     #[module]
///     pub struct Module {
///         #[export]
///         hidden: InternalType
///     }
/// }
///
/// #[injectable]
/// #[provider]
/// struct Provider {
///     #[import]
///     sub_mod: sub::Module
/// }
///
/// fn main() {
///     #[provider]
///     struct InitProvider;
///
///     let provider = InitProvider.provide::<Provider>();
///     let _facade = provider.provide::<sub::Facade>();
/// }
/// ```
#[proc_macro_attribute]
pub fn module(attr: TokenStream, item: TokenStream) -> TokenStream {
    handle_module(attr, item)
}
