#![no_std]
#![allow(clippy::needless_doctest_main)]
#![doc = include_str!("../README.md")]

#[cfg(feature = "macro")]
pub use nject_macro::{
    InjectableHelperAttr, ModuleHelperAttr, ProviderHelperAttr, ScopeHelperAttr, inject,
    injectable, key, module, provider,
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
/// #[provider]
/// struct InitProvider;
///
/// let provider = InitProvider.provide::<Provider>();
/// let facade = provider.provide::<sub::Facade>();
/// ```
pub trait Import<Module> {
    fn reference(&self) -> &Module;
}

/// A zero-cost wrapper that tags a `Value` with a `Name` type for named injection.
///
/// This allows multiple dependencies of the same type to coexist in a provider
/// by distinguishing them via a phantom type tag. At runtime, `Named<Name, Value>`
/// has the exact same memory layout as `Value` due to `#[repr(transparent)]`.
///
/// ```rust
/// use nject::{injectable, provider, Named};
///
/// // Define zero-sized tag types
/// struct DbUrl;
/// struct ApiKey;
///
/// #[provider]
/// #[provide(Named<DbUrl, String>, Named::new("postgres://localhost".into()))]
/// #[provide(Named<ApiKey, String>, Named::new("secret-key".into()))]
/// struct AppProvider;
///
/// let provider = AppProvider;
/// let db_url: Named<DbUrl, String> = provider.provide();
/// let api_key: Named<ApiKey, String> = provider.provide();
/// assert_eq!(*db_url, "postgres://localhost");
/// assert_eq!(*api_key, "secret-key");
/// ```
#[repr(transparent)]
pub struct Named<Name, Value> {
    /// The wrapped value.
    pub value: Value,
    _name: core::marker::PhantomData<Name>,
}

impl<Name, Value> Named<Name, Value> {
    /// Create a new `Named` value with the given tag.
    #[inline]
    pub const fn new(value: Value) -> Self {
        Self {
            value,
            _name: core::marker::PhantomData,
        }
    }

    /// Unwrap the `Named` value, discarding the tag.
    #[inline]
    pub fn into_inner(self) -> Value {
        self.value
    }
}

impl<Name, Value: core::fmt::Debug> core::fmt::Debug for Named<Name, Value> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.value.fmt(f)
    }
}

impl<Name, Value: Clone> Clone for Named<Name, Value> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new(self.value.clone())
    }
}

impl<Name, Value: Copy> Copy for Named<Name, Value> {}

impl<Name, Value: PartialEq> PartialEq for Named<Name, Value> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<Name, Value: Eq> Eq for Named<Name, Value> {}

impl<Name, Value> core::ops::Deref for Named<Name, Value> {
    type Target = Value;

    #[inline]
    fn deref(&self) -> &Value {
        &self.value
    }
}

impl<Name, Value> core::ops::DerefMut for Named<Name, Value> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Value {
        &mut self.value
    }
}

impl<Name, Value> From<Value> for Named<Name, Value> {
    #[inline]
    fn from(value: Value) -> Self {
        Self::new(value)
    }
}

/// A zero-sized type tag generated from a string key via FNV-1a hash.
///
/// Used with [`Named`] for string-based named injection. The `key!` macro
/// provides convenient syntax: `key!("db_url")` expands to `Key<{HASH}>`.
///
/// ```rust
/// use nject::{Named, Key, str_key_hash};
///
/// // These are equivalent:
/// type A = Named<Key<{str_key_hash("db_url")}>, String>;
///
/// let a: A = Named::new("hello".into());
/// assert_eq!(*a, "hello");
/// ```
pub struct Key<const K: u128>;

/// Compute an FNV-1a 128-bit hash of a string at compile time.
///
/// This uses the same algorithm as the internal module hashing,
/// producing a deterministic `u128` from any `&str`.
pub const fn str_key_hash(s: &str) -> u128 {
    // FNV-1a parameters for 128-bit
    // https://en.wikipedia.org/wiki/Fowler%E2%80%93Noll%E2%80%93Vo_hash_function#FNV_hash_parameters
    const FNV_OFFSET_BASIS: u128 = 0x6c62272e07bb014262b821756295c58d;
    const FNV_PRIME: u128 = 0x00000100000001b3;

    let bytes = s.as_bytes();
    let mut hash = FNV_OFFSET_BASIS;
    let mut i = 0;
    while i < bytes.len() {
        hash ^= bytes[i] as u128;
        hash = hash.wrapping_mul(FNV_PRIME);
        i += 1;
    }
    hash
}

/// For internal purposes only. Should not be used.
#[doc(hidden)]
pub trait RefInjectable<'prov, Value, Provider> {
    fn inject(&'prov self, provider: &'prov Provider) -> Value;
}

/// For internal purposes only. Should not be used.
#[doc(hidden)]
pub trait RefIterable<'prov, Value, Provider> {
    fn inject(&'prov self, provider: &'prov Provider, index: usize) -> Value;
}

/// For internal purposes only. Should not be used.
#[doc(hidden)]
pub trait Iterable<'prov, T> {
    fn iter(&'prov self) -> impl Iterator<Item = T>;
}
