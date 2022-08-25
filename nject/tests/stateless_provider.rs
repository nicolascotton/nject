use nject::provider;
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
