use nject::{init, injectable, module, provider};

#[test]
fn init_expr_without_modules_should_provide_target() {
    #[derive(Debug, PartialEq)]
    #[injectable]
    struct Dependency(#[inject(42)] i32);

    #[derive(Debug, PartialEq)]
    #[injectable]
    struct Service(Dependency);

    #[injectable]
    #[provider]
    struct Provider;

    let provider: Provider = init!();

    let service: Service = provider.provide();
    assert_eq!(service.0.0, 42);
}

#[test]
fn init_expr_with_single_module_should_provide_target() {
    #[injectable]
    #[module]
    #[export(i32, 42)]
    struct FooModule;

    #[injectable]
    #[provider]
    struct Provider(#[provide(i32, |x| *x)] i32);

    let provider: Provider = init!(FooModule);

    let value: i32 = provider.provide();
    assert_eq!(value, 42);
}

#[test]
fn init_expr_with_two_modules_should_chain_dependencies() {
    #[injectable]
    #[module]
    #[export(i32, 42)]
    struct ExprTwoConfigModule;

    #[injectable]
    #[module]
    #[export(String, |x: i32| format!("value: {}", x))]
    struct ExprTwoFormatModule;

    #[injectable]
    #[provider]
    struct AppProvider(#[provide(String, |x| x.to_owned())] String);

    let provider: AppProvider = init!(ExprTwoConfigModule, ExprTwoFormatModule);

    let greeting: String = provider.provide();
    assert_eq!(greeting, "value: 42");
}

#[test]
fn init_expr_with_three_modules_should_chain_all_dependencies() {
    #[injectable]
    #[module]
    #[export(i32, 100)]
    struct ExprThreeBaseModule;

    #[injectable]
    #[module]
    #[export(u32, |x: i32| x as u32 + 1)]
    struct ExprThreeMiddleModule;

    #[injectable]
    #[module]
    #[export(String, |x: u32| format!("result: {}", x))]
    struct ExprThreeTopModule;

    #[injectable]
    #[provider]
    struct Provider(
        #[import] ExprThreeBaseModule,
        #[import] ExprThreeMiddleModule,
        #[import] ExprThreeTopModule,
    );

    let provider: Provider = init!(
        ExprThreeBaseModule,
        ExprThreeMiddleModule,
        ExprThreeTopModule
    );

    let result: String = provider.provide();
    assert_eq!(result, "result: 101");
}

#[test]
fn init_expr_with_independent_modules_should_work() {
    #[injectable]
    #[module]
    #[export(i32, 10)]
    struct IndepModuleA;

    #[injectable]
    #[module]
    #[export(u32, 20)]
    struct IndepModuleB;

    #[injectable]
    #[provider]
    struct Provider(#[import] IndepModuleA, #[import] IndepModuleB);

    let provider: Provider = init!(IndepModuleA, IndepModuleB);

    let a: i32 = provider.provide();
    let b: u32 = provider.provide();
    assert_eq!(a, 10);
    assert_eq!(b, 20);
}

#[test]
fn init_block_without_modules_should_provide_target() {
    #[derive(Debug, PartialEq)]
    #[injectable]
    struct Dependency(#[inject(99)] i32);

    #[derive(Debug, PartialEq)]
    #[injectable]
    struct Service(Dependency);

    #[injectable]
    #[provider]
    struct Provider;

    init! {
        let provider: Provider;
    }

    let service: Service = provider.provide();
    assert_eq!(service.0.0, 99);
}

#[test]
fn init_block_without_modules_and_without_type_annotation_should_infer_type() {
    #[derive(Debug, PartialEq)]
    #[injectable]
    struct Dependency(#[inject(55)] i32);

    #[derive(Debug, PartialEq)]
    #[injectable]
    struct Service(Dependency);

    #[injectable]
    #[provider]
    struct Provider;

    init! {
        let provider;
    }

    let provider: Provider = provider;
    let service: Service = provider.provide();
    assert_eq!(service.0.0, 55);
}

#[test]
fn init_block_with_mixed_module_and_moduleless_declarations_should_work() {
    #[derive(Debug, PartialEq)]
    #[injectable]
    struct SimpleDependency(#[inject(7)] i32);

    #[derive(Debug, PartialEq)]
    #[injectable]
    struct SimpleService(SimpleDependency);

    #[injectable]
    #[provider]
    struct SimpleProvider;

    #[injectable]
    #[module]
    #[export(i32, 10)]
    struct ModuleProviderModule;

    #[injectable]
    #[provider]
    struct ModuleProvider(#[import] ModuleProviderModule);

    init! {
        let simple: SimpleProvider;
        let with_module: ModuleProvider = ModuleProviderModule;
    }

    let service: SimpleService = simple.provide();
    let value: i32 = with_module.provide();
    assert_eq!(service.0.0, 7);
    assert_eq!(value, 10);
}

#[test]
fn init_block_mut_without_modules_should_allow_mutation() {
    #[derive(Debug, PartialEq)]
    #[injectable]
    struct Service(#[inject(0)] i32);

    #[injectable]
    #[provider]
    struct Provider;

    init! {
        let mut provider: Provider;
    }
    init! {
        let provider2: Provider;
    }
    provider = provider2;

    let value: Service = provider.provide();
    assert_eq!(value.0, 0);
}

#[test]
fn init_block_with_single_module_should_provide_target() {
    #[injectable]
    #[module]
    #[export(i32, 99)]
    struct BlockSingleModule;

    #[injectable]
    #[provider]
    struct Provider(#[import] BlockSingleModule);

    init! {
        let provider: Provider = BlockSingleModule;
    }

    let value: i32 = provider.provide();
    assert_eq!(value, 99);
}

#[test]
fn init_block_with_two_modules_should_chain_dependencies() {
    #[injectable]
    #[module]
    #[export(i32, 7)]
    struct BlockTwoNumModule;

    #[injectable]
    #[module]
    #[export(String, |x: i32| format!("num={}", x))]
    struct BlockTwoFmtModule;

    #[injectable]
    #[provider]
    struct Provider(#[import] BlockTwoNumModule, #[import] BlockTwoFmtModule);

    init! {
        let provider: Provider = BlockTwoNumModule, BlockTwoFmtModule;
    }

    let s: String = provider.provide();
    assert_eq!(s, "num=7");
}

#[test]
fn init_block_without_type_annotation_should_infer_type() {
    #[injectable]
    #[module]
    #[export(i32, 55)]
    struct InferModule;

    #[injectable]
    #[provider]
    struct Provider(#[import] InferModule);

    init! {
        let provider: Provider = InferModule;
    }

    let value: i32 = provider.provide();
    assert_eq!(value, 55);
}

#[test]
fn init_block_mut_should_allow_mutation() {
    #[injectable]
    #[module]
    #[export(i32, 1)]
    struct MutModule;

    #[injectable]
    #[provider]
    struct Provider(#[import] MutModule);

    init! {
        let mut provider: Provider = MutModule;
    }
    let value: i32 = provider.provide();
    assert_eq!(value, 1);
    // Re-assign to prove mutability
    init! {
        let provider2: Provider = MutModule;
    }
    provider = provider2;
    let value2: i32 = provider.provide();
    assert_eq!(value2, 1);
}

#[test]
fn init_block_with_field_export_borrowing_should_work() {
    // Field-level #[export] produces &'prov T references, which borrow
    // from the intermediate provider. The block form keeps the
    // intermediates alive in the enclosing scope.

    #[derive(Debug, PartialEq)]
    #[injectable]
    struct InternalDep(#[inject(123)] i32);

    #[derive(Debug, PartialEq)]
    #[injectable]
    pub struct Facade<'a>(&'a InternalDep);

    #[injectable]
    #[module]
    struct DepModule {
        #[export]
        hidden: InternalDep,
    }

    #[injectable]
    #[provider]
    struct Provider(#[import] DepModule);

    init! {
        let provider: Provider = DepModule;
    }

    let facade: Facade = provider.provide();
    assert_eq!(facade.0.0, 123);
}

#[test]
fn init_block_with_cross_module_borrowing_should_work() {
    // Module A exports a field (reference).
    // Module B depends on Module A's export.
    // The block form keeps Module A's intermediate provider alive
    // so the references remain valid.

    #[derive(Debug, PartialEq)]
    #[injectable]
    struct CrossConfig(#[inject(42)] i32);

    #[injectable]
    #[module]
    struct CrossConfigModule {
        #[export]
        config: CrossConfig,
    }

    #[derive(Debug, PartialEq)]
    #[injectable]
    struct CrossService<'a>(&'a CrossConfig);

    #[injectable]
    #[module]
    struct CrossServiceModule<'a> {
        #[export]
        svc: CrossService<'a>,
    }

    #[injectable]
    #[provider]
    struct AppProvider<'a>(
        #[import] CrossConfigModule,
        #[import] CrossServiceModule<'a>,
    );

    init! {
        let provider: AppProvider<'_> = CrossConfigModule, CrossServiceModule;
    }

    let svc: CrossService = provider.provide();
    assert_eq!(svc.0.0, 42);
}

#[test]
fn init_block_with_multiple_declarations_should_work() {
    #[injectable]
    #[module]
    #[export(i32, 10)]
    struct AlphaModule;

    #[injectable]
    #[module]
    #[export(u32, 20)]
    struct BetaModule;

    #[injectable]
    #[provider]
    struct AlphaProvider(#[import] AlphaModule);

    #[injectable]
    #[provider]
    struct BetaProvider(#[import] BetaModule);

    init! {
        let alpha: AlphaProvider = AlphaModule;
        let beta: BetaProvider = BetaModule;
    }

    let a: i32 = alpha.provide();
    let b: u32 = beta.provide();
    assert_eq!(a, 10);
    assert_eq!(b, 20);
}

#[test]
fn init_block_with_multiple_chained_declarations_should_work() {
    // Multiple declarations where each has its own dependency chain.

    #[injectable]
    #[module]
    #[export(i32, 10)]
    struct MultiChainNumModule;

    #[injectable]
    #[module]
    #[export(String, |x: i32| format!("n={}", x))]
    struct MultiChainFmtModule;

    #[injectable]
    #[module]
    #[export(u64, 999)]
    struct MultiChainIdModule;

    #[injectable]
    #[provider]
    struct FmtProvider(#[import] MultiChainNumModule, #[import] MultiChainFmtModule);

    #[injectable]
    #[provider]
    struct IdProvider(#[import] MultiChainIdModule);

    init! {
        let fmt_prov: FmtProvider = MultiChainNumModule, MultiChainFmtModule;
        let id_prov: IdProvider = MultiChainIdModule;
    }

    let s: String = fmt_prov.provide();
    let id: u64 = id_prov.provide();
    assert_eq!(s, "n=10");
    assert_eq!(id, 999);
}

#[test]
fn init_block_with_multiple_declarations_and_borrowing_should_work() {
    // Multiple declarations where one uses field-level exports (borrowing).

    #[derive(Debug, PartialEq)]
    #[injectable]
    struct DbPool(#[inject(5)] i32);

    #[injectable]
    #[module]
    struct DbModule {
        #[export]
        pool: DbPool,
    }

    #[derive(Debug, PartialEq)]
    #[injectable]
    struct Repo<'a>(&'a DbPool);

    #[injectable]
    #[provider]
    struct DbProvider(#[import] DbModule);

    #[injectable]
    #[module]
    #[export(u16, 8080)]
    struct HttpModule;

    #[injectable]
    #[provider]
    struct HttpProvider(#[import] HttpModule);

    init! {
        let db: DbProvider = DbModule;
        let http: HttpProvider = HttpModule;
    }

    let repo: Repo = db.provide();
    let port: u16 = http.provide();
    assert_eq!(repo.0.0, 5);
    assert_eq!(port, 8080);
}

#[test]
fn init_block_with_three_module_chain_should_work() {
    // Single declaration with three modules forming a dependency chain:
    // BaseModule exports i32 → MiddleModule uses i32 to export u64 → TopModule uses u64 to export String

    #[injectable]
    #[module]
    #[export(i32, 100)]
    struct BlockThreeBaseModule;

    #[injectable]
    #[module]
    #[export(u64, |x: i32| x as u64 * 3)]
    struct BlockThreeMiddleModule;

    #[injectable]
    #[module]
    #[export(String, |x: u64| format!("final={}", x))]
    struct BlockThreeTopModule;

    #[injectable]
    #[provider]
    struct FullProvider(
        #[import] BlockThreeBaseModule,
        #[import] BlockThreeMiddleModule,
        #[import] BlockThreeTopModule,
    );

    init! {
        let provider: FullProvider = BlockThreeBaseModule, BlockThreeMiddleModule, BlockThreeTopModule;
    }

    let i: i32 = provider.provide();
    let u: u64 = provider.provide();
    let s: String = provider.provide();
    assert_eq!(i, 100);
    assert_eq!(u, 300);
    assert_eq!(s, "final=300");
}

#[test]
fn init_block_with_field_exports_and_three_modules_should_work() {
    // Three modules using field-level exports (references) in a chain.

    #[derive(Debug, PartialEq)]
    #[injectable]
    struct FieldThreeConfig(#[inject(7)] i32);

    #[injectable]
    #[module]
    struct FieldThreeConfigModule {
        #[export]
        cfg: FieldThreeConfig,
    }

    #[derive(Debug, PartialEq)]
    #[injectable]
    struct FieldThreeCache<'a>(#[inject(99)] i32, &'a FieldThreeConfig);

    #[injectable]
    #[module]
    struct FieldThreeCacheModule<'a> {
        #[export]
        cache: FieldThreeCache<'a>,
    }

    #[derive(Debug, PartialEq)]
    #[injectable]
    struct FieldThreeHandler<'a>(&'a FieldThreeConfig, &'a FieldThreeCache<'a>);

    #[injectable]
    #[provider]
    struct AppProvider<'a>(
        #[import] FieldThreeConfigModule,
        #[import] FieldThreeCacheModule<'a>,
    );

    init! {
        let provider: AppProvider<'_> = FieldThreeConfigModule, FieldThreeCacheModule;
    }

    let cfg: &FieldThreeConfig = provider.provide();
    let cache: &FieldThreeCache = provider.provide();
    let handler: FieldThreeHandler = provider.provide();
    assert_eq!(cfg.0, 7);
    assert_eq!(cache.0, 99);
    assert_eq!(handler.0.0, 7);
    assert_eq!(handler.1.0, 99);
}

#[test]
fn init_block_paren_syntax_should_still_work() {
    // The old paren syntax `init!(let name = ...)` should still work
    #[injectable]
    #[module]
    #[export(i32, 77)]
    struct ParenModule;

    #[injectable]
    #[provider]
    struct Provider(#[import] ParenModule);

    init!(let provider: Provider = ParenModule);

    let value: i32 = provider.provide();
    assert_eq!(value, 77);
}
