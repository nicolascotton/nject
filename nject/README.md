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
nject = "0.4"
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
struct Provider {
    #[provide]
    shared: DepOne,
}

fn main() {
    let provider = Provider { shared: DepOne };
    let _facade: Facade = provider.provide();
}

```
### Works with dyn traits
```rust
use nject::{injectable, provider};
use std::rc::Rc;

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
    rc_dep: Rc<dyn Greeter>,
}

#[provider]
#[provide(Box<dyn Greeter>, |greeter: GreeterOne| Box::new(greeter))]
struct Provider {
    #[provide(dyn Greeter)]
    greeter: GreeterOne,
    #[provide(Rc<dyn Greeter>, |x| x.clone())]
    rc_greeter: Rc<GreeterOne>,
}

fn main() {
    let provider = Provider { 
        greeter: GreeterOne,
        rc_greeter: Rc::new(GreeterOne),
    };
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

#[inject(|injectable_dep: InjectableFromInjectAttr| Self { 
    non_injectable_value: injectable_dep.non_injectable_value + 10, 
    injectable_dep 
})]
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
    #[inject(|injectable_dep: InjectableFromInjectAttr| PartiallyInjectable {
        non_injectable_value: 111, 
        injectable_dep 
    })]
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

    trait Greeter {
        fn greet(&self) -> &str;
    }

    #[injectable]
    struct GreeterOne;

    impl Greeter for GreeterOne {
        fn greet(&self) -> &str {
            "One"
        }
    }

    #[injectable]
    struct InternalType(#[inject(123)] i32); // Not visible outside of module.

    #[injectable]
    pub struct Facade<'a> {
        hidden: &'a InternalType,
        hidden_dyn: &'a dyn Greeter,
    }

    #[injectable]
    #[module]
    pub struct Module {
        // Internal shared type exports must be made on fields (not the struct).
        #[export]
        hidden: InternalType,
        #[export(dyn Greeter)]
        hidden_dyn: GreeterOne,
    }
}

#[injectable]
#[provider]
struct Provider {
    #[import]
    sub_mod: sub::Module,
}

fn main() {
    #[provider]
    struct InitProvider;

    let provider = InitProvider.provide::<Provider>();
    let _facade = provider.provide::<sub::Facade>();
}

```
#### Limitations
1. Internal dependencies can only be exported by a single module.
1. Generic parameters are not supported on modules.

### Use modules to export public dependencies
```rust
use nject::{injectable, provider};

mod sub {
    use nject::{injectable, module};
    use std::boxed::Box;
    use std::rc::Rc;

    pub trait Greeter {
        fn greet(&self) -> &str;
    }

    #[injectable]
    struct GreeterOne;

    impl Greeter for GreeterOne {
        fn greet(&self) -> &str {
            "One"
        }
    }

    #[injectable]
    pub struct Facade<'a> {
        public_box: Box<dyn Greeter>,
        public_rc: Rc<dyn Greeter>,
        public_i32: &'a i32,
    }

    #[injectable]
    // The absolute public path to access the module. 
    // If no path is given, the struct name will be used and must be unique across all modules.
    // Keywords like `crate` and `Self` will be substituted accordingly.
    #[module(crate::sub::Self)]
    // Public type exports must be made on the struct (not the fields). 
    // To prevent name collisions, use absolute paths in types.
    #[export(std::boxed::Box<dyn crate::sub::Greeter>, |x: GreeterOne| Box::new(x))]
    #[export(std::rc::Rc<dyn crate::sub::Greeter>, self.public.clone())]
    #[export(&'prov i32, &123)]
    pub struct Module {
        #[inject(|x: GreeterOne| Rc::new(x))]
        public: Rc<dyn Greeter>,
    }
}

#[injectable]
#[provider]
struct Provider {
    #[import]
    // To import module public exports, use the absolute path given in its definition.
    sub_mod: crate::sub::Module,
}

fn main() {
    #[provider]
    struct InitProvider;

    let provider = InitProvider.provide::<Provider>();
    let _facade = provider.provide::<sub::Facade>();
}

```
#### Limitations
1. Public exports are discovered as macros expand. Therefore, modules must expand before their use in any providers.
   - This limitation is only applicable if **both** module and provider are defined in the same crate. 
1. Requires `cargo` to build. Run `cargo clean -p nject-macro` to clean the cache if it ever gets corrupted.
1. Generic parameters are not supported on modules.

### Use scopes to scope dependencies
```rust
use nject::{injectable, module, provider};

#[injectable]
struct ModuleDep;

#[injectable]
#[module]
struct ScopeModule {
    #[export]
    module_dep: ModuleDep,
}

#[injectable]
struct RootDep;

#[injectable]
struct ScopeDep;

#[injectable]
struct ScopeFacade<'a> {
    root_dep: &'a RootDep, 
    scope_dep: &'a ScopeDep,
    scope_module_dep: &'a ModuleDep,
}

#[injectable]
#[provider]
#[scope(ScopeDep)]
#[scope(#[import] ScopeModule)]
#[scope(other: #[arg] &'scope ScopeDep)]
#[scope(other: #[arg] &'scope ModuleDep)]
struct Provider(#[provide] RootDep);

fn main() {
    #[provider]
    struct InitProvider;

    let provider = InitProvider.provide::<Provider>();
    let scope = provider.scope();
    let scope_facade = scope.provide::<ScopeFacade>();

    let other_scope = provider.other_scope(scope_facade.scope_dep, scope_facade.scope_module_dep);
    let _other_scope_facade = other_scope.provide::<ScopeFacade>();
}
```

## Examples
You can look into the [axum](https://github.com/nicolascotton/nject/tree/main/examples/axum)/[actix](https://github.com/nicolascotton/nject/tree/main/examples/actix) example for a Web API use case or into the [Leptos](https://github.com/nicolascotton/nject/tree/main/examples/leptos) example for a Web App.
## Credits
- [Syn](https://github.com/dtolnay/syn) - [MIT](https://github.com/dtolnay/syn/blob/master/LICENSE-MIT) or [Apache-2.0](https://github.com/dtolnay/syn/blob/master/LICENSE-APACHE)
- [Quasi-Quoting](https://github.com/dtolnay/quote) - [MIT](https://github.com/dtolnay/quote/blob/master/LICENSE-MIT) or [Apache-2.0](https://github.com/dtolnay/quote/blob/master/LICENSE-APACHE)
- [Rust](https://github.com/rust-lang/rust) - [MIT](https://github.com/rust-lang/rust/blob/master/LICENSE-MIT) or [Apache-2.0](https://github.com/rust-lang/rust/blob/master/LICENSE-APACHE)
