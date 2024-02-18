#![doc = include_str!("../README.md")]

#[cfg(feature = "macro")]
pub use nject_macro::{
    inject, injectable, module, provider, InjectableHelperAttr, ModuleHelperAttr,
    ProviderHelperAttr, ScopeHelperAttr,
};

/// Provide a value for a specified type. Should be used with the `provide` macro for a better experience.
/// ```rust
/// use nject::{injectable, provider};
///
/// struct DependencyToProvide {
///     value: i32,
/// }
///
/// struct SharedDependencyToProvide {
///     value: i32,
/// }
///
/// #[injectable]
/// struct Facade<'a>(DependencyToProvide, &'a SharedDependencyToProvide);
///
/// #[provider]
/// #[provide(DependencyToProvide, DependencyToProvide { value: 42 })]
/// struct Provider {
///     #[provide]
///     shared: SharedDependencyToProvide
/// }
///
/// fn main() {
///     let provider = Provider { shared: SharedDependencyToProvide { value: 123 } };
///     let _facade: Facade = provider.provide();
/// }
/// ```
pub trait Provider<'prov, Value> {
    fn provide(&'prov self) -> Value;
}

/// Inject dependencies for a specific type and return its value. Should be used with the `injectable` macro for a better experience.
/// ```rust
/// use nject::{injectable, provider};
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

/// Import exportations made from a module. Should be used with the `import` macro for a better experience.
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
pub trait Import<Module> {
    fn reference(&self) -> &Module;
}

/// For internal purposes only. Should not be used.
pub trait RefInjectable<'prov, Value, Provider> {
    fn inject(&'prov self, provider: &'prov Provider) -> Value;
}
