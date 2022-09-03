# nject ![Rust](https://github.com/nicolascotton/nject/workflows/Rust/badge.svg)
Simple zero cost injection library made for rust
## Install
Add the following to your `Cargo.toml`:
```toml
[dependencies]
nject = { git = "https://github.com/nicolascotton/nject.git" }
```
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
    let _facade: Facade = Provider.inject();
}

```
### Works with lifetimes - enables shared dependencies
```rust
use nject::{injectable, provide, provider};

#[injectable]
struct DepOne;

#[injectable]
struct Facade<'a> {
    dep: &'a DepOne,
}

#[provider]
#[provide(&'a DepOne, self.shared)]
struct Provider<'a> {
    shared: &'a DepOne,
}

fn main() {
    let provider = Provider { shared: &DepOne };
    let _facade: Facade = provider.inject();
}

```
### Works with dyn traits
```rust
use nject::{injectable, provide, provider};

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
#[provide(Box<dyn Greeter>, Box::<GreeterOne>::new(self.inject()))]
#[provide(&'prov dyn Greeter, &self.greeter)]
struct Provider {
    greeter: GreeterOne,
}

fn main() {
    let provider = Provider { greeter: GreeterOne };
    let _facade: Facade = provider.inject();
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
    let _facade: Facade<DepOne> = Provider.inject();
}

```
### Works with generic providers
```rust
use nject::{injectable, provide, provider};

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
#[provide(&'a dyn Greeter, self.0)]
struct Provider<'a, T: Greeter>(&'a T);

fn main() {
    let _dev_facade: Facade = Provider(&DevGreeter).inject();
    let _prod_facade: Facade = Provider(&ProdGreeter).inject();
}
```
## Examples
You can look into the [axum example](https://github.com/nicolascotton/nject/tree/main/examples/axum) for a web API use case.
## Credits
- [Syn](https://github.com/dtolnay/syn) - [MIT](https://github.com/dtolnay/syn/blob/master/LICENSE-MIT) or [Apache-2.0](https://github.com/dtolnay/syn/blob/master/LICENSE-APACHE)
- [Quasi-Quoting](https://github.com/dtolnay/quote) - [MIT](https://github.com/dtolnay/quote/blob/master/LICENSE-MIT) or [Apache-2.0](https://github.com/dtolnay/quote/blob/master/LICENSE-APACHE)
- [Rust](https://github.com/rust-lang/rust) - [MIT](https://github.com/rust-lang/rust/blob/master/LICENSE-MIT) or [Apache-2.0](https://github.com/rust-lang/rust/blob/master/LICENSE-APACHE)