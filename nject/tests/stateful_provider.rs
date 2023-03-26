mod common;
use common::*;
use nject::{injectable, provide, provider};
use std::rc::Rc;

#[provider]
#[provide(&'a StructWithoutDeps, self.lifetime)]
#[provide(Box<dyn Greeter>, Box::<GreeterOne>::new(self.provide()))]
#[provide(Rc<i32>, self.shared_rc.clone())]
#[provide(&'prov dyn Greeter, &self.greeter)]
struct Provider<'a> {
    lifetime: &'a StructWithoutDeps,
    shared_rc: Rc<i32>,
    greeter: GreeterTwo,
}

impl<'a> Provider<'a> {
    fn new() -> Self {
        Self {
            lifetime: &StructWithoutDeps,
            shared_rc: Rc::new(123),
            greeter: GreeterTwo,
        }
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

#[test]
fn provide_ref_dyn_trait_without_boxing_should_give_corresponding_ref() {
    // Given
    let provider = Provider::new();
    // When
    let greeter: &dyn Greeter = provider.provide();
    // Then
    assert_eq!(greeter.greet(), GreeterTwo.greet());
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
