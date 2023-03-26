#![doc = include_str!("../README.md")]

#[cfg(feature = "macro")]
pub use nject_macro::{
    inject, injectable, provide, provider, InjectableHelperAttr, ProviderHelperAttr,
};

/// Provide a value for a specified type. Should be used with the `provide` macro for a better experience.
/// ```rust
/// use nject::{injectable, provide, provider};
///
/// struct DependencyToProvide {
///     value: i32,
/// }
///
/// #[injectable]
/// struct Facade(DependencyToProvide);
///
/// #[provider]
/// #[provide(DependencyToProvide, DependencyToProvide { value: 42 })]
/// struct Provider;
///
/// fn main() {
///     let _facade: Facade = Provider.provide();
/// }
/// ```
pub trait Provider<'prov, Value> {
    fn provide(&'prov self) -> Value;
}

/// Inject dependencies for a specific type and return its value. Should be used with the `injectable` macro for a better experience.
/// ```rust
/// use nject::{injectable, provide, provider};
///
/// struct Dependency {
///     value: i32,
/// }
///
/// #[injectable]
/// struct Facade {
///     #[inject(Dependency { value: 42 })]
///     dep: Dependency
/// }
///
/// #[provider]
/// struct Provider;
///
/// fn main() {
///     let _facade: Facade = Provider.provide();
/// }
/// ```
pub trait Injectable<'prov, Injecty, Provider> {
    fn inject(provider: &'prov Provider) -> Injecty;
}
