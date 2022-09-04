use nject::{inject, injectable, provider};
mod common;
pub use common::*;

#[provider]
struct Provider;

#[test]
fn provide_struct_without_deps_should_give_corresponding_struct() {
    // Given
    let provider = Provider {};
    // When
    let value: StructWithoutDeps = provider.provide();
    // Then
    assert_eq!(value, StructWithoutDeps {});
}

#[test]
fn provide_struct_with_named_deps_should_give_corresponding_struct() {
    // Given
    let provider = Provider {};
    // When
    let value: StructWithNamedDeps = provider.provide();
    // Then
    assert_eq!(
        value,
        StructWithNamedDeps {
            dep: StructWithoutDeps {}
        }
    );
}

#[test]
fn provide_struct_with_unnamed_deps_should_give_corresponding_struct() {
    // Given
    let provider = Provider {};
    // When
    let value: StructWithUnnamedDeps = provider.provide();
    // Then
    assert_eq!(value, StructWithUnnamedDeps(StructWithoutDeps));
}

#[test]
fn provide_struct_with_named_generic_deps_should_give_corresponding_struct() {
    // Given
    let provider = Provider {};
    // When
    let value: StructWithNamedGenericDeps<StructWithoutDeps> = provider.provide();
    // Then
    assert_eq!(
        value,
        StructWithNamedGenericDeps {
            dep: StructWithoutDeps
        }
    );
}

#[test]
fn provide_struct_with_unnamed_generic_deps_should_give_corresponding_struct() {
    // Given
    let provider = Provider {};
    // When
    let value: StructWithUnnamedGenericDeps<StructWithoutDeps> = provider.provide();
    // Then
    assert_eq!(value, StructWithUnnamedGenericDeps(StructWithoutDeps));
}

#[test]
fn provide_struct_with_non_injectable_named_deps_with_inject_annotation_should_give_corresponding_struct(
) {
    // Given
    let provider = Provider {};
    // When
    let value: StructWithNonInjectableNamedDeps = provider.provide();
    // Then
    assert_eq!(
        value,
        StructWithNonInjectableNamedDeps {
            dep: NonInjectableStruct { value: 123 }
        }
    );
}

#[test]
fn provide_struct_with_non_injectable_named_deps_and_inject_annotation_should_prioritize_annotation_on_struct(
) {
    // Given
    let provider = Provider {};
    // When
    let value: StructWithNonInjectableNamedDepsAndInjectAttr = provider.provide();
    // Then
    assert_eq!(
        value,
        StructWithNonInjectableNamedDepsAndInjectAttr {
            non_inj_dep: NonInjectableStruct { value: 456 },
            dep: StructWithoutDeps
        }
    );
}

#[test]
fn provide_struct_with_non_injectable_unnamed_deps_with_inject_annotation_should_give_corresponding_struct(
) {
    // Given
    let provider = Provider {};
    // When
    let value: StructWithNonInjectableUnnamedDeps = provider.provide();
    // Then
    assert_eq!(
        value,
        StructWithNonInjectableUnnamedDeps(NonInjectableStruct { value: 123 })
    );
}

#[test]
fn provide_struct_with_non_injectable_unnnamed_deps_and_inject_annotation_should_prioritize_annotation_on_struct(
) {
    // Given
    let provider = Provider {};
    // When
    let value: StructWithNonInjectableUnnamedDepsAndInjectAttr = provider.provide();
    // Then
    assert_eq!(
        value,
        StructWithNonInjectableUnnamedDepsAndInjectAttr(
            NonInjectableStruct { value: 456 },
            StructWithoutDeps
        )
    );
}

#[injectable]
#[derive(Debug, PartialEq)]
struct StructWithNonInjectableNamedDeps {
    dep: NonInjectableStruct,
}

#[inject(Self { value: 123 })]
#[derive(Debug, PartialEq)]
struct NonInjectableStruct {
    value: i32,
}

#[injectable]
#[derive(Debug, PartialEq)]
struct StructWithNonInjectableNamedDepsAndInjectAttr {
    #[inject(NonInjectableStruct { value: 456 })]
    non_inj_dep: NonInjectableStruct,
    dep: StructWithoutDeps,
}

#[injectable]
#[derive(Debug, PartialEq)]
struct StructWithNonInjectableUnnamedDeps(NonInjectableStruct);

#[injectable]
#[derive(Debug, PartialEq)]
struct StructWithNonInjectableUnnamedDepsAndInjectAttr(
    #[inject( NonInjectableStruct { value: 456 })] NonInjectableStruct,
    StructWithoutDeps,
);
