use nject::{injectable, module, provider};
use std::vec;

#[provider]
struct InitProvider;

#[test]
fn iter_with_multiple_exports_for_a_type_from_different_modules_should_return_iterable_of_exports()
{
    // Given
    #[module]
    #[injectable]
    #[export(&'prov str, "First")]
    struct MultiExportFirstModuleStr;
    #[module]
    #[injectable]
    #[export(&'prov str, "Last")]
    struct MultiExportLastModuleStr;
    #[injectable]
    #[provider]
    struct Provider(
        #[import] MultiExportFirstModuleStr,
        #[import] MultiExportLastModuleStr,
    );
    let provider = InitProvider.provide::<Provider>();
    // When
    let values = provider.iter::<&str>().collect::<Vec<_>>();
    // Then
    assert_eq!(values, vec!["First", "Last"])
}

#[test]
fn provide_with_multiple_exports_for_a_type_from_different_modules_should_return_value_from_last_module(
) {
    // Given
    #[module]
    #[injectable]
    #[export(&'prov str, "First")]
    struct MultiExportFirstModuleStr;
    #[module]
    #[injectable]
    #[export(&'prov str, "Last")]
    struct MultiExportLastModuleStr;
    #[injectable]
    #[provider]
    struct Provider(
        #[import] MultiExportFirstModuleStr,
        #[import] MultiExportLastModuleStr,
    );
    let provider = InitProvider.provide::<Provider>();
    // When
    let value = provider.provide::<&str>();
    // Then
    assert_eq!(value, "Last")
}

#[test]
fn iter_with_multiple_exports_for_a_type_from_same_modules_should_return_iterable_of_exports() {
    // Given
    #[module]
    #[injectable]
    #[export(&'prov str, "First")]
    #[export(&'prov str, "Last")]
    struct MultiExportSameModuleStr;
    #[injectable]
    #[provider]
    struct Provider(#[import] MultiExportSameModuleStr);
    let provider = InitProvider.provide::<Provider>();
    // When
    let values = provider.iter::<&str>().collect::<Vec<_>>();
    // Then
    assert_eq!(values, vec!["First", "Last"])
}

#[test]
fn provide_with_multiple_exports_for_a_type_from_same_modules_should_return_last_one() {
    // Given
    #[module]
    #[injectable]
    #[export(&'prov str, "First")]
    #[export(&'prov str, "Last")]
    struct MultiExportSameModuleStr;
    #[injectable]
    #[provider]
    struct Provider(#[import] MultiExportSameModuleStr);
    let provider = InitProvider.provide::<Provider>();
    // When
    let value = provider.provide::<&str>();
    // Then
    assert_eq!(value, "Last")
}

#[test]
fn iter_with_multiple_exports_module_imported_from_scope_should_return_iterable_of_exports() {
    // Given
    #[module]
    #[injectable]
    #[export(i32, 1)]
    #[export(i32, 2)]
    struct MultiExportScopeModule;

    #[injectable]
    #[provider]
    #[scope(#[import] MultiExportScopeModule)]
    struct Root;

    let scope = Root.scope();
    // When
    let values = scope.iter::<i32>().collect::<Vec<_>>();
    // Then
    assert_eq!(values, vec![1, 2]);
}

#[test]
fn iter_with_multiple_exports_module_imported_from_root_should_return_iterable_of_exports() {
    // Given
    #[module]
    #[injectable]
    #[export(i32, 1)]
    #[export(i32, 2)]
    struct MultiExportRootModule;

    #[injectable]
    #[derive(PartialEq, Debug)]
    struct ScopedDep(i32);

    #[injectable]
    #[provider]
    #[scope(ScopedDep)]
    struct Root(#[import] MultiExportRootModule);

    let root = InitProvider.provide::<Root>();
    let scope = root.scope();
    // When
    let values = scope.iter::<i32>().collect::<Vec<_>>();
    // Then
    assert_eq!(values, vec![1, 2]);
    assert_eq!(scope.provide::<ScopedDep>(), ScopedDep(2));
}
