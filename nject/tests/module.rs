use crate::sub::Greeter;
use nject::{init, injectable, module, provider};
use std::rc::Rc;

#[test]
fn provide_with_simple_module_should_export_its_members_correctly() {
    // Given
    #[injectable]
    #[provider]
    struct Provider(#[import] crate::sub::SimpleModule);
    let provider: Provider = init!();
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
    struct Provider<'a>(#[import] &'a crate::sub::SimpleModule);
    let module: sub::SimpleModule = init!();
    let provider = Provider(&module);
    // When
    let facade = provider.provide::<sub::SimpleFacade>();
    // Then
    assert_eq!(facade, sub::expected_simple_facade(provider.0))
}

#[test]
fn provide_with_generic_module_should_export_its_members_correctly() {
    // Given
    #[injectable]
    #[provider]
    #[provide(&'prov i32, &self.0)]
    struct SeedProvider(#[inject(123)] i32);
    #[injectable]
    #[provider]
    struct Provider<'a, T>(#[import] crate::sub::GenericModule<'a, T>);
    let init_prov: SeedProvider = init!();
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
    struct Provider<'a>(#[import] crate::sub::ModuleWithRef<'a>);
    let provider: Provider = init!();
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
    struct Provider(#[import] crate::sub::DynDepModule);
    let provider: Provider = init!();
    // When
    let dep = provider.provide::<&dyn sub::Greeter>();
    // Then
    assert_eq!(dep.greet(), sub::GreeterOne.greet())
}

#[injectable]
#[module]
#[export(std::rc::Rc<i32>, self.0.clone())]
struct TestModule1(#[inject(Rc::new(123))] Rc<i32>);

#[test]
fn provide_with_module_with_external_type_export_with_simple_factory_should_provide_its_members_correctly()
 {
    // Given
    #[injectable]
    #[provider]
    struct Provider(#[import] crate::TestModule1);
    let provider: Provider = init!();
    // When
    let dep = provider.provide::<Rc<i32>>();
    // Then
    assert_eq!(*dep, 123)
}

#[test]
fn provide_with_module_with_external_type_export_with_complex_factory_should_provide_its_members_correctly()
 {
    // Given
    #[injectable]
    #[module]
    #[export(Box<i32>, |x: i32| Box::new(x))]
    struct TestModule2;
    #[injectable]
    #[provider]
    #[provide(i32, 123)]
    struct Provider(#[import] TestModule2);
    let provider: Provider = init!();
    // When
    let dep = provider.provide::<Box<i32>>();
    // Then
    assert_eq!(*dep, 123)
}

#[test]
fn provide_with_module_with_ref_external_type_export_should_provide_its_members_correctly() {
    // Given
    #[injectable]
    #[module]
    #[export(&'prov i32, &self.0)]
    struct TestModule3(#[inject(123)] i32);
    #[injectable]
    #[provider]
    struct Provider(#[import] TestModule3);
    let provider: Provider = init!();
    // When
    let dep = provider.provide::<&i32>();
    // Then
    assert_eq!(dep, &123)
}

#[test]
fn provide_with_module_with_factory_internal_export_should_provide_its_members_correctly() {
    // Given
    #[derive(Debug, PartialEq)]
    struct Ref<T: PartialEq>(Rc<T>);
    #[injectable]
    #[module]
    struct Module(
        #[inject(Rc::new(123))]
        #[export(Ref<i32>, |x| Ref(x.clone()))]
        Rc<i32>,
        #[inject(Ref(Rc::new(456)))]
        #[export(&'prov Ref<i32>, |x| &x)]
        Ref<i32>,
    );
    #[injectable]
    #[provider]
    struct Provider(#[import] Module);
    let provider: Provider = init!();
    // When
    let dep = provider.provide::<Ref<i32>>();
    let dep_ref = provider.provide::<&Ref<i32>>();
    // Then
    assert_eq!(dep, Ref(Rc::new(123)));
    assert!(Rc::ptr_eq(&dep.0, &provider.0.0));
    assert_eq!(*dep_ref.0, *provider.0.1.0);
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
        SimpleRefFacade(module.hidden)
    }

    pub fn expected_generic_facade<'a, T>(
        module: &'a GenericModule<'a, T>,
    ) -> GenericFacade<'a, T> {
        GenericFacade(&module.hidden)
    }
}
