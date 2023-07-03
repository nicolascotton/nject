<div align="center">
  <h1>nject</h1>
</div>
<div align="center">
  <!-- Build status -->
  <a href="https://github.com/nicolascotton/nject">
    <img src="https://github.com/nicolascotton/nject/workflows/Rust/badge.svg" />
  </a>
  <!-- Crates version -->
  <a href="https://crates.io/crates/nject">
    <img src="https://img.shields.io/crates/v/nject.svg"
    alt="Crates.io version" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/nject">
    <img src="https://img.shields.io/crates/d/nject.svg"
      alt="Download" />
  </a>
  <!-- docs -->
  <a href="https://docs.rs/nject">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg"
      alt="docs.rs docs" />
  </a>
</div>
<br />

Simple zero cost dependency injection library made for rust
## Install
Add the following to your `Cargo.toml`:
```toml
[dependencies]
nject = "0.3"
```
## Why `nject`?
- **Zero cost**: Using this library is equivalent to manually injecting your dependencies as shown in the [benchmarks](https://github.com/nicolascotton/nject/tree/main/examples/benchmark).
- **Compile time only**: If configured incorrectly, `nject` will fail at compile time.

## Use cases
### Removes the need to specify dependencies across your modules
```rust
use nject::{injectable, provider};

#[injectable]
struct DepOne;

#[injectable]
struct DepTwo {
    dep: DepOne,
}

#[injectable]
struct Facade {
    dep: DepTwo,
}

#[provider]
struct Provider;

fn main() {
    let _facade: Facade = Provider.provide();
}

```
### Works with lifetimes - enables shared dependencies
```rust
use nject::{injectable, provider};

struct DepOne;

#[injectable]
struct Facade<'a> {
    dep: &'a DepOne,
}

#[provider]
struct Provider<'a> {
    #[provide]
    shared: &'a DepOne,
}

fn main() {
    let provider = Provider { shared: &DepOne };
    let _facade: Facade = provider.provide();
}

```
### Works with dyn traits
```rust
use nject::{injectable, provider};

trait Greeter {
    fn greet(&self);
}

#[injectable]
struct GreeterOne;

impl Greeter for GreeterOne {
    fn greet(&self) {
        println!("Greeting");
    }
}

#[injectable]
struct Facade<'a> {
    boxed_dep: Box<dyn Greeter>,
    ref_dep: &'a dyn Greeter,
}

#[provider]
#[provide(Box<dyn Greeter>, Box::<GreeterOne>::new(self.provide()))]
struct Provider {
    #[provide(dyn Greeter)]
    greeter: GreeterOne,
}

fn main() {
    let provider = Provider { greeter: GreeterOne };
    let _facade: Facade = provider.provide();
}

```
### Works with generics
```rust
use nject::{injectable, provider};

#[injectable]
struct DepOne;

#[injectable]
struct Facade<T> {
    dep: T,
}

#[provider]
struct Provider;

fn main() {
    let _facade: Facade<DepOne> = Provider.provide();
}

```
### Works with generic providers
```rust
use nject::{injectable, provider};

trait Greeter {
    fn greet(&self);
}

#[injectable]
struct DevGreeter;

impl Greeter for DevGreeter {
    fn greet(&self) {
        println!("Greeting Dev");
    }
}

#[injectable]
struct ProdGreeter;

impl Greeter for ProdGreeter {
    fn greet(&self) {
        println!("Greeting production");
    }
}

#[injectable]
struct Facade<'a> {
    dep: &'a dyn Greeter,
}

#[provider]
struct Provider<'a, T: Greeter>(#[provide(dyn Greeter)] &'a T);

fn main() {
    let _dev_facade: Facade = Provider(&DevGreeter).provide();
    let _prod_facade: Facade = Provider(&ProdGreeter).provide();
}
```
### Easily inject non-injectable dependencies
```rust
use nject::{inject, injectable, provider};

#[inject(Self { non_injectable_value: 123 })]
struct InjectableFromInjectAttr {
    non_injectable_value: i32,
}

struct NonInjectable {
    non_injectable_value: i32,
}

#[inject(Self { 
    non_injectable_value: injectable_dep.non_injectable_value + 10, 
    injectable_dep 
}, injectable_dep: InjectableFromInjectAttr)]
struct PartiallyInjectable {
    non_injectable_value: i32,
    injectable_dep: InjectableFromInjectAttr
}

#[injectable]
struct Facade {
    dep_from_injected: InjectableFromInjectAttr,
    dep_from_partial_inject: PartiallyInjectable,
    #[inject(NonInjectable { non_injectable_value: 456 })]
    dep_from_inject_attr: NonInjectable,
    #[inject(InjectableFromInjectAttr { non_injectable_value: 789 })]
    dep_from_inject_attr_override: InjectableFromInjectAttr,
    #[inject(PartiallyInjectable {
        non_injectable_value: 111, 
        injectable_dep 
    }, injectable_dep: InjectableFromInjectAttr)]
    dep_from_partial_inject_attr_override: PartiallyInjectable,
}

#[provider]
struct Provider;

fn main() {
    let _facade = Provider.provide::<Facade>();
}
```
### Use modules to export internal shared dependencies
```rust
use nject::{injectable, provider};

mod sub {
    use nject::{injectable, module};

    #[injectable]
    struct InternalType( #[inject(123)] i32); // Not visible outside of module.

    #[injectable]
    pub struct Facade<'a> {
        hidden: &'a InternalType
    }

    #[injectable]
    #[module]
    pub struct Module {
        #[export]
        hidden: InternalType
    }
}

#[injectable]
#[provider]
struct Provider {
    #[import]
    subModule: sub::Module
}

fn main() {
    #[provider]
    struct InitProvider;

    let provider = InitProvider.provide::<Provider>();
    let _facade = provider.provide::<sub::Facade>();
}

```
### Limitations
1. Dependencies can only be exported by a single module.
1. Modules can only export types defined in its crate.
1. Generic parameters are not supported on modules.

## Examples
You can look into the [axum](https://github.com/nicolascotton/nject/tree/main/examples/axum) example for a Web API use case or into the [Leptos](https://github.com/nicolascotton/nject/tree/main/examples/leptos) example for a Web App.
## Credits
- [Syn](https://github.com/dtolnay/syn) - [MIT](https://github.com/dtolnay/syn/blob/master/LICENSE-MIT) or [Apache-2.0](https://github.com/dtolnay/syn/blob/master/LICENSE-APACHE)
- [Quasi-Quoting](https://github.com/dtolnay/quote) - [MIT](https://github.com/dtolnay/quote/blob/master/LICENSE-MIT) or [Apache-2.0](https://github.com/dtolnay/quote/blob/master/LICENSE-APACHE)
- [Rust](https://github.com/rust-lang/rust) - [MIT](https://github.com/rust-lang/rust/blob/master/LICENSE-MIT) or [Apache-2.0](https://github.com/rust-lang/rust/blob/master/LICENSE-APACHE)
