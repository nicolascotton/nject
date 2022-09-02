# nject ![Rust](https://github.com/nicolascotton/nject/workflows/Rust/badge.svg)
Simple zero cost injection library for rust
## Install
Add the following to your `Cargo.toml`:
```toml
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
    let _facade: Facade = Provider.provide();
}

```
### Works with lifetimes - enables shared dependencies
```rust
use nject::{injectable, provider};

#[injectable]
struct DepOne;

#[injectable]
struct Facade<'a> {
    dep: &'a DepOne,
}

#[provider]
struct Provider<'a> {
    shared: &'a DepOne,
}

impl<'a> nject::Provider<'_, &'a DepOne> for Provider<'a> {
    fn provide(&self) -> &'a DepOne {
        self.shared
    }
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
struct Facade {
    dep: Box<dyn Greeter>,
}

#[provider]
struct Provider;

impl nject::Provider<'_, Box<dyn Greeter>> for Provider {
    fn provide(&self) -> Box<dyn Greeter> {
        Box::<GreeterOne>::new(self.provide())
    }
}

fn main() {
    let _facade: Facade = Provider.provide();
}

```
### Works with generics
```rust
use nject::{injectable, provider};

#[injectable]
struct DepOne;

#[injectable]
struct Facade<T> {
    dep: T
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
    dep: &'a dyn Greeter
}

#[provider]
struct Provider<'a, T: Greeter>(&'a T);

impl<'a, T: Greeter> nject::Provider<'_, &'a dyn Greeter> for Provider<'a, T> {
    fn provide(&self) -> &'a dyn Greeter {
        self.0
    }
}

fn main() {
    let _dev_facade: Facade = Provider(&DevGreeter).provide();
    let _prod_facade: Facade = Provider(&ProdGreeter).provide();
}
```
## Credits
- [Rust](https://github.com/rust-lang/rust) - [MIT](https://github.com/rust-lang/rust/blob/master/LICENSE-MIT) or [Apache-2.0](https://github.com/rust-lang/rust/blob/master/LICENSE-APACHE)