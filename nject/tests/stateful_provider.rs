mod common;
pub use common::*;
use nject::{injectable, provider};
use std::rc::Rc;

#[provider]
#[provide(&'a StructWithoutDeps, self.lifetime)]
#[provide(Box<dyn Greeter>, |greeter: GreeterOne| Box::new(greeter))]
#[provide(&'prov dyn Greeter, &self.greeter)]
struct Provider<'a> {
    lifetime: &'a StructWithoutDeps,
    #[provide(Rc<i32>, |x| x.clone())]
    shared_rc: Rc<i32>,
    greeter: GreeterTwo,
    #[provide(i32, |x| *x)]
    i32_ref: &'a i32,
}

impl<'a> Provider<'a> {
    fn new() -> Self {
        Self {
            lifetime: &StructWithoutDeps,
            shared_rc: Rc::new(123),
            greeter: GreeterTwo,
            i32_ref: &123,
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

#[test]
fn provide_with_provide_attr_on_field_should_give_corresponding_ref() {
    // Given
    #[provider]
    struct Provider(#[provide] i32);
    let provider = Provider(123);
    // When
    let value: &i32 = provider.provide();
    // Then
    assert_eq!(value, &provider.0);
}

#[test]
fn provide_with_provide_attr_on_ref_field_should_give_corresponding_ref() {
    // Given
    #[provider]
    struct Provider<'a>(#[provide] &'a i32);
    let provider = Provider(&123);
    // When
    let value: &i32 = provider.provide();
    // Then
    assert_eq!(value, provider.0);
}

#[test]
fn provide_with_provide_attr_on_value_field_with_specified_dyn_trait_should_give_corresponding_ref()
{
    // Given
    #[provider]
    struct Provider<T: Greeter>(#[provide(dyn Greeter)] T);
    let provider = Provider(GreeterTwo);
    // When
    let value: &dyn Greeter = provider.provide();
    // Then
    assert_eq!(value.greet(), provider.0.greet());
}

#[test]
fn provide_with_provide_attr_on_ref_field_with_specified_dyn_trait_should_give_corresponding_ref() {
    // Given
    #[provider]
    struct Provider<'a, T: Greeter>(#[provide(dyn Greeter)] &'a T);
    let provider = Provider(&GreeterOne);
    // When
    let value: &dyn Greeter = provider.provide();
    // Then
    assert_eq!(value.greet(), provider.0.greet());
}

#[test]
fn provide_with_value_factory_should_give_corresponding_factory() {
    // Given
    #[injectable]
    #[derive(Debug, PartialEq)]
    struct Dep(#[inject(123)] i32);

    #[provider]
    struct Provider;
    // When
    let provider = Provider.provide::<&dyn nject::Provider<Dep>>();
    // Then
    assert_eq!(provider.provide(), Dep(123));
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
