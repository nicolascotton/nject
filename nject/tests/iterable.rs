use nject::{injectable, module, provider};

#[provider]
struct InitProvider;

#[test]
fn iter_with_multiple_export_for_a_type_from_different_modules_should_return_iterable_of_exports() {
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
fn provide_with_multiple_export_for_a_type_from_different_modules_should_return_value_from_last_module(
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
fn iter_with_multiple_export_for_a_type_from_same_modules_should_return_iterable_of_exports() {
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
fn provide_with_multiple_export_for_a_type_from_same_modules_should_return_last_one() {
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
