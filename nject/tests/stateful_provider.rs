use std::rc::Rc;

use nject::{injectable, provider};
mod common;
pub use common::*;

#[provider]
struct Provider<'a> {
    lifetime: &'a StructWithoutDeps,
    shared_rc: Rc<i32>,
}

impl<'a> Provider<'a> {
    fn new() -> Self {
        Self {
            lifetime: &StructWithoutDeps,
            shared_rc: Rc::new(123),
        }
    }
}

impl<'a> nject::Provider<'_, &'a StructWithoutDeps> for Provider<'a> {
    fn provide(&self) -> &'a StructWithoutDeps {
        self.lifetime
    }
}

impl<'a> nject::Provider<'_, Box<dyn Greeter>> for Provider<'a> {
    fn provide(&self) -> Box<dyn Greeter> {
        Box::<GreeterOne>::new(self.provide())
    }
}

impl<'a> nject::Provider<'_, Rc<i32>> for Provider<'a> {
    fn provide(&self) -> Rc<i32> {
        self.shared_rc.clone()
    }
}

#[test]
fn provide_struct_with_lifetime_deps_should_give_corresponding_struct() {
    // Given
    let provider = Provider::new();
    // When
    let value: StructWithNamedDepAndLifetime = provider.provide();
    // Then
    assert_eq!(
        value,
        StructWithNamedDepAndLifetime {
            dep: &StructWithoutDeps
        }
    );
}

#[test]
fn provide_dyn_trait_with_boxed_dyn_should_give_corresponding_box() {
    // Given
    let provider = Provider::new();
    // When
    let greeter: Box<dyn Greeter> = provider.provide();
    // Then
    assert_eq!(greeter.greet(), GreeterOne.greet());
}

#[test]
fn provide_shared_rc_should_give_same_instance() {
    // Given
    let provider = Provider::new();
    // When
    let shared_one: Rc<i32> = provider.provide();
    let shared_two: Rc<i32> = provider.provide();
    // Then
    assert!(Rc::ptr_eq(&shared_one, &shared_two));
}

trait Greeter {
    fn greet(&self) -> String;
}

#[injectable]
struct GreeterOne;
impl Greeter for GreeterOne {
    fn greet(&self) -> String {
        "Greeting One".into()
    }
}

#[injectable]
struct GreeterTwo;
impl Greeter for GreeterTwo {
    fn greet(&self) -> String {
        "Greeting Two".into()
    }
}
