use nject::{inject, injectable, module, provider};

#[test]
fn provide_with_type_providable_by_root_should_be_providable_by_scope() {
    // Given
    #[provider]
    #[provide(i32, 123)]
    #[scope(i32)]
    struct Root;

    let scope = Root.scope();
    // When
    let value = scope.provide::<i32>();
    // Then
    assert_eq!(value, Root.provide());
}

#[test]
fn provide_with_scope_type_providable_by_root_should_expose_a_ref_to_the_type_in_scope() {
    // Given
    #[provider]
    #[provide(i32, 123)]
    #[scope(i32)]
    struct Root;

    let scope = Root.scope();
    // When
    let value = scope.provide::<&i32>();
    // Then
    assert_eq!(*value, Root.provide());
}

#[test]
fn provide_with_imported_scoped_module_type_providable_by_root_should_scope_module() {
    // Given
    #[module]
    #[injectable]
    struct Module(#[export] Integer);

    #[provider]
    #[scope(#[import] Module)]
    struct Root;

    let scope = Root.scope();
    // When
    let value = scope.provide::<&Integer>();
    // Then
    assert_eq!(value, &Integer(123));
}

#[test]
fn provide_with_scoped_dyn_type_should_give_corresponding_value() {
    // Given
    #[provider]
    #[scope(#[provide(dyn IntegerOwner)] Integer)]
    struct Root;

    let scope = Root.scope();
    // When
    let value = scope.provide::<&dyn IntegerOwner>();
    // Then
    assert_eq!(value.value(), 123);
}

#[test]
fn provide_with_scoped_type_should_give_corresponding_value() {
    // Given
    #[derive(PartialEq, Debug)]
    #[inject(Self(123))]
    struct ScopedDep(i32);

    #[derive(PartialEq, Debug)]
    #[inject(Self(456))]
    struct RootDep(i32);

    #[derive(PartialEq, Debug)]
    #[injectable]
    struct ScopedValue<'a>(&'a RootDep, &'a ScopedDep);

    #[provider]
    #[injectable]
    #[scope(ScopedDep)]
    struct Root(#[provide] RootDep);

    #[provider]
    struct InitProvider;

    let root = InitProvider.provide::<Root>();
    let scope = root.scope();
    // When
    let value = scope.provide::<ScopedValue>();
    // Then
    assert_eq!(value, ScopedValue(&root.0, &scope.0));
}

#[test]
fn provide_with_lifetime_on_scoped_type_should_give_corresponding_value() {
    // Given
    #[derive(PartialEq, Debug)]
    #[inject(Self(&123))]
    struct ScopedDep<'a>(&'a i32);

    #[derive(PartialEq, Debug)]
    #[inject(Self(456))]
    struct RootDep(i32);

    #[derive(PartialEq, Debug)]
    #[injectable]
    struct ScopedValue<'a>(&'a RootDep, &'a ScopedDep<'a>);

    #[provider]
    #[injectable]
    #[scope(ScopedDep<'scope>)]
    struct Root(#[provide] RootDep);

    #[provider]
    struct InitProvider;

    let root = InitProvider.provide::<Root>();
    let scope = root.scope();
    // When
    let value = scope.provide::<ScopedValue>();
    // Then
    assert_eq!(value, ScopedValue(&root.0, &scope.0));
}

#[test]
fn provide_with_generic_root_should_give_corresponding_value() {
    // Given
    #[inject(Self(&123))]
    struct ScopedDep<'a>(&'a i32);

    struct RootDep(i32);
    impl IntegerOwner for RootDep {
        fn value(&self) -> i32 {
            self.0
        }
    }

    #[injectable]
    struct ScopedValue<'a>(&'a dyn IntegerOwner, &'a ScopedDep<'a>);

    #[provider]
    #[injectable]
    #[scope(ScopedDep<'scope>)]
    struct Root<'a, T: IntegerOwner>(#[provide(dyn IntegerOwner)] &'a T);

    let root = Root(&RootDep(456));
    let scope = root.scope();
    // When
    let value = scope.provide::<ScopedValue>();
    // Then
    assert_eq!(value.0.value(), 456);
}

trait IntegerOwner {
    fn value(&self) -> i32;
}

#[inject(Self(123))]
#[derive(Debug, PartialEq)]
struct Integer(i32);

impl IntegerOwner for Integer {
    fn value(&self) -> i32 {
        self.0
    }
}