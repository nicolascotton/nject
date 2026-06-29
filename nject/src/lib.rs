#![no_std]
#![allow(clippy::needless_doctest_main)]
#![doc = include_str!("../README.md")]

#[cfg(feature = "macro")]
pub use nject_macro::{
    __nject_finalize_imports, InjectableHelperAttr, ModuleHelperAttr, ProviderHelperAttr,
    ScopeHelperAttr, init, inject, injectable, module, provider,
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
/// let provider = Provider { shared: SharedDependencyToProvide { value: 123 } };
/// let facade: Facade = provider.provide();
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
/// let _facade: Facade = Provider.provide();
/// ```
pub trait Injectable<'prov, Injecty, Provider> {
    fn inject(provider: &'prov Provider) -> Injecty;
}

/// Import exportations made from a module. Should be used with the `import` macro for a better experience.
/// ```rust
/// use nject::{init, injectable, provider};
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
///     pub struct SubModule {
///         #[export]
///         hidden: InternalType
///     }
/// }
///
/// #[injectable]
/// #[provider]
/// struct Provider(#[import] crate::sub::SubModule);
///
/// fn main() {
///     let provider: Provider = init!();
///     let facade = provider.provide::<sub::Facade>();
/// }
/// ```
pub trait Import<Module> {
    fn reference(&self) -> &Module;
}

/// For internal purposes only. Should not be used.
#[doc(hidden)]
pub trait RefInjectable<'prov, Value, Provider> {
    fn inject(&'prov self, provider: &'prov Provider) -> Value;
}

impl<'prov, 'module, Value, Provider, Module> RefInjectable<'prov, Value, Provider>
    for &'module Module
where
    'module: 'prov,
    Module: RefInjectable<'prov, Value, Provider>,
{
    #[inline]
    fn inject(&'prov self, provider: &'prov Provider) -> Value {
        (**self).inject(provider)
    }
}

/// For internal purposes only. Should not be used.
#[doc(hidden)]
pub trait RefIterable<'prov, Value, Provider> {
    fn inject(&'prov self, provider: &'prov Provider, index: usize) -> Value;
}

impl<'prov, 'module, Value, Provider, Module> RefIterable<'prov, Value, Provider>
    for &'module Module
where
    'module: 'prov,
    Module: RefIterable<'prov, Value, Provider>,
{
    #[inline]
    fn inject(&'prov self, provider: &'prov Provider, index: usize) -> Value {
        (**self).inject(provider, index)
    }
}

/// For internal purposes only. Should not be used.
#[doc(hidden)]
pub trait Iterable<'prov, T> {
    fn iter(&'prov self) -> impl Iterator<Item = T>;
}

/// For internal purposes only. Should not be used.
/// Orchestrates chaining through module macros, accumulating exports.
#[doc(hidden)]
#[macro_export]
macro_rules! __nject_next {
    // Recursive case: more modules to process
    (
        next = [($next_macro:path, [$($next_field:tt)*], [$($next_args:tt)*]) $(, ($($remaining:tt)*))*],
        exports = [$($exports:tt)*],
        $($provider_info:tt)*
    ) => {
        $next_macro! {
            @nject_collect
            next = [$(($($remaining)*)),*],
            field = [$($next_field)*],
            module_args = [$($next_args)*],
            exports = [$($exports)*],
            $($provider_info)*
        }
    };
    // Base case: no more modules → call finalizer
    (
        next = [],
        exports = [$($exports:tt)*],
        $($provider_info:tt)*
    ) => {
        ::nject::__nject_finalize_imports! {
            exports = [$($exports)*],
            $($provider_info)*
        }
    };
}
