//! # nject ![Rust](https://github.com/nicolascotton/nject/workflows/Rust/badge.svg)
//! Simple zero cost dependency injection library made for rust
//! ## Install
//! Add the following to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! nject = { git = "https://github.com/nicolascotton/nject.git" }
//! ```
//! ## Use cases
//! ### Removes the need to specify dependencies across your modules
//! ```rust
//! use nject::{injectable, provider};
//!
//! #[injectable]
//! struct DepOne;
//!
//! #[injectable]
//! struct DepTwo {
//!     dep: DepOne,
//! }
//!
//! #[injectable]
//! struct Facade {
//!     dep: DepTwo,
//! }
//!
//! #[provider]
//! struct Provider;
//!
//! fn main() {
//!     let _facade: Facade = Provider.inject();
//! }
//!
//! ```
//! ### Works with lifetimes - enables shared dependencies
//! ```rust
//! use nject::{injectable, provide, provider};
//!
//! #[injectable]
//! struct DepOne;
//!
//! #[injectable]
//! struct Facade<'a> {
//!     dep: &'a DepOne,
//! }
//!
//! #[provider]
//! #[provide(&'a DepOne, self.shared)]
//! struct Provider<'a> {
//!     shared: &'a DepOne,
//! }
//!
//! fn main() {
//!     let provider = Provider { shared: &DepOne };
//!     let _facade: Facade = provider.inject();
//! }
//!
//! ```
//! ### Works with dyn traits
//! ```rust
//! use nject::{injectable, provide, provider};
//!
//! trait Greeter {
//!     fn greet(&self);
//! }
//!
//! #[injectable]
//! struct GreeterOne;
//!
//! impl Greeter for GreeterOne {
//!     fn greet(&self) {
//!         println!("Greeting");
//!     }
//! }
//!
//! #[injectable]
//! struct Facade<'a> {
//!     boxed_dep: Box<dyn Greeter>,
//!     ref_dep: &'a dyn Greeter,
//! }
//!
//! #[provider]
//! #[provide(Box<dyn Greeter>, Box::<GreeterOne>::new(self.inject()))]
//! #[provide(&'prov dyn Greeter, &self.greeter)]
//! struct Provider {
//!     greeter: GreeterOne,
//! }
//!
//! fn main() {
//!     let provider = Provider { greeter: GreeterOne };
//!     let _facade: Facade = provider.inject();
//! }
//!
//! ```
//! ### Works with generics
//! ```rust
//! use nject::{injectable, provider};
//!
//! #[injectable]
//! struct DepOne;
//!
//! #[injectable]
//! struct Facade<T> {
//!     dep: T,
//! }
//!
//! #[provider]
//! struct Provider;
//!
//! fn main() {
//!     let _facade: Facade<DepOne> = Provider.inject();
//! }
//!
//! ```
//! ### Works with generic providers
//! ```rust
//! use nject::{injectable, provide, provider};
//!
//! trait Greeter {
//!     fn greet(&self);
//! }
//!
//! #[injectable]
//! struct DevGreeter;
//!
//! impl Greeter for DevGreeter {
//!     fn greet(&self) {
//!         println!("Greeting Dev");
//!     }
//! }
//!
//! #[injectable]
//! struct ProdGreeter;
//!
//! impl Greeter for ProdGreeter {
//!     fn greet(&self) {
//!         println!("Greeting production");
//!     }
//! }
//!
//! #[injectable]
//! struct Facade<'a> {
//!     dep: &'a dyn Greeter,
//! }
//!
//! #[provider]
//! #[provide(&'a dyn Greeter, self.0)]
//! struct Provider<'a, T: Greeter>(&'a T);
//!
//! fn main() {
//!     let _dev_facade: Facade = Provider(&DevGreeter).inject();
//!     let _prod_facade: Facade = Provider(&ProdGreeter).inject();
//! }
//! ```
//! ### Easily inject non-injectable dependencies
//! ```rust
//! use nject::{inject, injectable, provide, provider};
//!
//! #[inject(Self { non_injectable_value: 123 })]
//! struct NonInjectableWithInjectAttr {
//!     non_injectable_value: i32,
//! }
//!
//! struct NonInjectable {
//!     non_injectable_value: i32,
//! }
//!
//! #[injectable]
//! struct Facade {
//!     dep_from_injected: NonInjectableWithInjectAttr,
//!     #[inject(NonInjectable { non_injectable_value: 456 })]
//!     dep_from_inject_attr: NonInjectable,
//!     #[inject(NonInjectableWithInjectAttr { non_injectable_value: 789 })]
//!     dep_from_inject_attr_override: NonInjectableWithInjectAttr,
//! }
//!
//! #[provider]
//! struct Provider;
//!
//! fn main() {
//!     let _facade = Provider.inject::<Facade>();
//! }
//! ```
//! ## Examples
//! You can look into the [axum](https://github.com/nicolascotton/nject/tree/main/examples/axum) example for a web API use case.
//! ## Credits
//! - [Syn](https://github.com/dtolnay/syn) - [MIT](https://github.com/dtolnay/syn/blob/master/LICENSE-MIT) or [Apache-2.0](https://github.com/dtolnay/syn/blob/master/LICENSE-APACHE)
//! - [Quasi-Quoting](https://github.com/dtolnay/quote) - [MIT](https://github.com/dtolnay/quote/blob/master/LICENSE-MIT) or [Apache-2.0](https://github.com/dtolnay/quote/blob/master/LICENSE-APACHE)
//! - [Rust](https://github.com/rust-lang/rust) - [MIT](https://github.com/rust-lang/rust/blob/master/LICENSE-MIT) or [Apache-2.0](https://github.com/rust-lang/rust/blob/master/LICENSE-APACHE)

#[cfg(feature = "macro")]
pub use nject_macro::{inject, injectable, provide, provider, InjectableHelperAttr};

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
