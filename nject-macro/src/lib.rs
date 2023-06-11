#![doc = include_str!("../README.md")]
mod inject;
mod injectable;
mod module;
mod provide;
mod provider;
use inject::handle_inject;
use injectable::handle_injectable;
use module::handle_module;
use proc_macro::TokenStream;
use provide::handle_provide;
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
#[proc_macro_derive(ProviderHelperAttr, attributes(import))]
pub fn provider_helper_attr(_item: TokenStream) -> TokenStream {
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
/// #[inject(Self(12, injectable_dep), injectable_dep: DepOne)]
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

/// Attribute to mark a struct as a provider.
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
pub fn provider(_attr: TokenStream, item: TokenStream) -> TokenStream {
    handle_provider(item)
}

/// Attribute to provide a given instance for a specific type.
/// ```rust
/// use nject::{injectable, provide, provider};
///
/// struct Dependency {
///     value: i32,
/// }
///
/// #[injectable]
/// struct Facade(Dependency);
///
/// #[provider]
/// #[provide(Dependency, Dependency { value: 123 })]
/// struct Provider;
///
/// fn main() {
///     let _dependency: Dependency = Provider.provide();
///     let _facade: Facade = Provider.provide();
/// }
/// ```
#[proc_macro_attribute]
pub fn provide(attr: TokenStream, item: TokenStream) -> TokenStream {
    handle_provide(attr, item)
}

/// Declare a module to export internal types.
/// ```rust
/// use nject::{injectable, provider};
///
/// mod sub {
///     use nject::{injectable, module};
///
///     #[injectable]
///     struct InternalType( #[inject(123)] i32); // Not visible outside of module.
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
///     subModule: sub::Module
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
