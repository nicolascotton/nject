use nject::{injectable, provider};

use crate::sub::Greeter;

#[provider]
struct InitProvider;

#[test]
fn provide_with_simple_module_should_export_its_members_correctly() {
    // Given
    #[injectable]
    #[provider]
    struct Provider(#[import] sub::SimpleModule);
    let provider = InitProvider.provide::<Provider>();
    // When
    let facade = provider.provide::<sub::SimpleFacade>();
    // Then
    assert_eq!(facade, sub::expected_simple_facade(&provider.0))
}

#[test]
fn provide_with_ref_to_module_should_export_its_members_correctly() {
    // Given
    #[injectable]
    #[provider]
    struct Provider<'a>(#[import] &'a sub::SimpleModule);
    let module = InitProvider.provide::<sub::SimpleModule>();
    let provider = Provider(&module);
    // When
    let facade = provider.provide::<sub::SimpleFacade>();
    // Then
    assert_eq!(facade, sub::expected_simple_facade(&provider.0))
}

#[test]
fn provide_with_generic_module_should_export_its_members_correctly() {
    // Given
    #[provider]
    #[provide(&'prov i32, &self.0)]
    struct InitProvider(i32);
    #[injectable]
    #[provider]
    struct Provider<'a, T>(#[import] sub::GenericModule<'a, T>);
    let init_prov = InitProvider(123);
    let provider = init_prov.provide::<Provider<i32>>();
    // When
    let facade = provider.provide::<sub::GenericFacade<_>>();
    // Then
    assert_eq!(facade, sub::expected_generic_facade(&provider.0))
}

#[test]
fn provide_with_module_with_ref_dep_should_export_its_members_correctly() {
    // Given
    #[injectable]
    #[provider]
    struct Provider<'a>(#[import] sub::ModuleWithRef<'a>);
    let provider = InitProvider.provide::<Provider>();
    // When
    let facade = provider.provide::<sub::SimpleRefFacade>();
    // Then
    assert_eq!(facade, sub::expected_simple_ref_facade(&provider.0))
}

#[test]
fn provide_with_module_with_dyn_dep_should_export_its_members_correctly() {
    // Given
    #[injectable]
    #[provider]
    struct Provider(#[import] sub::DynDepModule);
    let provider = InitProvider.provide::<Provider>();
    // When
    let dep = provider.provide::<&dyn sub::Greeter>();
    // Then
    assert_eq!(dep.greet(), sub::GreeterOne.greet())
}

mod sub {
    use nject::{injectable, module};

    pub trait Greeter {
        fn greet(&self) -> &str;
    }

    #[injectable]
    pub struct GreeterOne;

    impl Greeter for GreeterOne {
        fn greet(&self) -> &str {
            "One"
        }
    }

    static REF_DEP: &SimpleRefDep = &SimpleRefDep(123);

    #[derive(PartialEq, Debug)]
    #[injectable]
    struct SimpleDep(#[inject(123)] i32);

    #[derive(PartialEq, Debug)]
    #[injectable]
    struct SimpleRefDep(#[inject(123)] i32);

    #[derive(PartialEq, Debug)]
    #[injectable]
    pub struct SimpleFacade<'a>(&'a SimpleDep);

    #[derive(PartialEq, Debug)]
    #[injectable]
    pub struct SimpleRefFacade<'a>(&'a SimpleRefDep);

    #[injectable]
    #[module]
    pub struct SimpleModule {
        #[export]
        hidden: SimpleDep,
    }

    #[injectable]
    #[module]
    pub struct DynDepModule {
        #[export(dyn Greeter)]
        dyn_dep: GreeterOne,
    }

    #[derive(PartialEq, Debug)]
    #[injectable]
    struct GenericDep<'a, T>(&'a T);

    #[derive(PartialEq, Debug)]
    #[injectable]
    pub struct GenericFacade<'a, T>(&'a GenericDep<'a, T>);

    #[injectable]
    #[module]
    pub struct GenericModule<'a, T> {
        #[export]
        hidden: GenericDep<'a, T>,
    }

    #[injectable]
    #[module]
    pub struct ModuleWithRef<'a> {
        #[export]
        #[inject(REF_DEP)]
        hidden: &'a SimpleRefDep,
    }

    pub fn expected_simple_facade<'a>(module: &'a SimpleModule) -> SimpleFacade<'a> {
        SimpleFacade(&module.hidden)
    }

    pub fn expected_simple_ref_facade<'a>(module: &'a ModuleWithRef) -> SimpleRefFacade<'a> {
        SimpleRefFacade(&module.hidden)
    }

    pub fn expected_generic_facade<'a, T>(
        module: &'a GenericModule<'a, T>,
    ) -> GenericFacade<'a, T> {
        GenericFacade(&module.hidden)
    }
}
