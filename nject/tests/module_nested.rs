use nject::init;

mod modules {
    use nject::{injectable, module};

    #[derive(PartialEq, Debug)]
    #[injectable]
    pub struct Dep(#[inject(42)] pub i32);

    #[injectable]
    #[module]
    pub struct MyModule {
        #[export]
        pub hidden: Dep,
    }

    pub mod inner {
        use nject::{injectable, module};

        #[derive(PartialEq, Debug)]
        #[injectable]
        pub struct InnerDep(#[inject(7)] pub i32);

        #[injectable]
        #[module]
        pub struct InnerModule {
            #[export]
            pub hidden: InnerDep,
        }
    }
}

mod providers {
    use nject::{injectable, provider};

    use crate::modules::Dep;
    use crate::modules::inner::InnerDep;

    #[injectable]
    pub struct Facade<'a>(pub &'a Dep);

    #[injectable]
    pub struct InnerFacade<'a>(pub &'a InnerDep);

    // Imports a module from a sibling top-level module via `crate::…`.
    #[injectable]
    #[provider]
    pub struct CrateProvider(#[import] pub crate::modules::MyModule);

    // Imports a deeply nested module via `crate::…::…`.
    #[injectable]
    #[provider]
    pub struct DeepProvider(#[import] pub crate::modules::inner::InnerModule);

    // Imports a module from the parent module via `super::…`.
    pub mod inner {
        use nject::{injectable, provider};

        #[injectable]
        #[provider]
        pub struct SuperProvider(#[import] pub super::super::modules::MyModule);
    }
}

// Provider and module declared in the same module – tests the bare/`self`
// resolution paths.
mod same_scope {
    use nject::{injectable, module, provider};

    #[derive(PartialEq, Debug)]
    #[injectable]
    pub struct LocalDep(#[inject(99)] pub i32);

    #[injectable]
    #[module]
    pub struct LocalModule {
        #[export]
        pub hidden: LocalDep,
    }

    #[injectable]
    pub struct LocalFacade<'a>(pub &'a LocalDep);

    // Single-segment path – resolved via the textually in-scope macro.
    #[injectable]
    #[provider]
    pub struct BareProvider(#[import] pub LocalModule);

    // `self::…` path – resolved via the local-alias `pub(crate) use`.
    #[injectable]
    #[provider]
    pub struct SelfProvider(#[import] pub self::LocalModule);
}

#[test]
fn nested_provider_can_import_via_crate_path() {
    let provider: providers::CrateProvider = init!();
    let facade = provider.provide::<providers::Facade>();
    assert_eq!(facade.0.0, 42);
}

#[test]
fn nested_provider_can_import_deep_crate_path() {
    let provider: providers::DeepProvider = init!();
    let facade = provider.provide::<providers::InnerFacade>();
    assert_eq!(facade.0.0, 7);
}

#[test]
fn nested_provider_can_import_via_super_path() {
    let provider: providers::inner::SuperProvider = init!();
    let facade = provider.provide::<providers::Facade>();
    assert_eq!(facade.0.0, 42);
}

#[test]
fn provider_can_import_module_in_same_scope_by_bare_name() {
    let provider: same_scope::BareProvider = init!();
    let facade = provider.provide::<same_scope::LocalFacade>();
    assert_eq!(facade.0.0, 99);
}

#[test]
fn provider_can_import_module_in_same_scope_via_self_path() {
    let provider: same_scope::SelfProvider = init!();
    let facade = provider.provide::<same_scope::LocalFacade>();
    assert_eq!(facade.0.0, 99);
}
