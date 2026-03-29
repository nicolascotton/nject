use nject::{init, injectable, module, provider};

#[test]
fn init_expr_with_single_module_should_provide_target() {
    #[injectable]
    #[module]
    #[export(i32, 42)]
    struct FooModule;

    #[injectable]
    #[provider]
    struct Provider(#[import] FooModule);

    let provider: Provider = init!(FooModule);

    let value: i32 = provider.provide();
    assert_eq!(value, 42);
}

#[test]
fn init_expr_with_two_modules_should_chain_dependencies() {
    #[injectable]
    #[module]
    #[export(i32, 42)]
    struct ConfigModule;

    #[injectable]
    #[module]
    #[export(String, |x: i32| format!("value: {}", x))]
    struct FormatModule;

    #[injectable]
    #[provider]
    struct AppProvider(#[import] ConfigModule, #[import] FormatModule);

    let provider: AppProvider = init!(ConfigModule, FormatModule);

    let greeting: String = provider.provide();
    assert_eq!(greeting, "value: 42");
}

#[test]
fn init_expr_with_three_modules_should_chain_all_dependencies() {
    #[injectable]
    #[module]
    #[export(i32, 100)]
    struct BaseModule;

    #[injectable]
    #[module]
    #[export(u32, |x: i32| x as u32 + 1)]
    struct MiddleModule;

    #[injectable]
    #[module]
    #[export(String, |x: u32| format!("result: {}", x))]
    struct TopModule;

    #[injectable]
    #[provider]
    struct Provider(
        #[import] BaseModule,
        #[import] MiddleModule,
        #[import] TopModule,
    );

    let provider: Provider = init!(BaseModule, MiddleModule, TopModule);

    let result: String = provider.provide();
    assert_eq!(result, "result: 101");
}

#[test]
fn init_expr_with_independent_modules_should_work() {
    #[injectable]
    #[module]
    #[export(i32, 10)]
    struct ModuleA;

    #[injectable]
    #[module]
    #[export(u32, 20)]
    struct ModuleB;

    #[injectable]
    #[provider]
    struct Provider(#[import] ModuleA, #[import] ModuleB);

    let provider: Provider = init!(ModuleA, ModuleB);

    let a: i32 = provider.provide();
    let b: u32 = provider.provide();
    assert_eq!(a, 10);
    assert_eq!(b, 20);
}

// ── Block form: init! { let name = M1, M2; } ───────────────────────

#[test]
fn init_block_with_single_module_should_provide_target() {
    #[injectable]
    #[module]
    #[export(i32, 99)]
    struct SingleModule;

    #[injectable]
    #[provider]
    struct Provider(#[import] SingleModule);

    init! {
        let provider: Provider = SingleModule;
    }

    let value: i32 = provider.provide();
    assert_eq!(value, 99);
}

#[test]
fn init_block_with_two_modules_should_chain_dependencies() {
    #[injectable]
    #[module]
    #[export(i32, 7)]
    struct NumModule;

    #[injectable]
    #[module]
    #[export(String, |x: i32| format!("num={}", x))]
    struct FmtModule;

    #[injectable]
    #[provider]
    struct Provider(#[import] NumModule, #[import] FmtModule);

    init! {
        let provider: Provider = NumModule, FmtModule;
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
    struct Config(#[inject(42)] i32);

    #[injectable]
    #[module]
    struct ConfigModule {
        #[export]
        config: Config,
    }

    #[derive(Debug, PartialEq)]
    #[injectable]
    struct Service<'a>(&'a Config);

    #[injectable]
    #[module]
    struct ServiceModule<'a> {
        #[export]
        svc: Service<'a>,
    }

    #[injectable]
    #[provider]
    struct AppProvider<'a>(#[import] ConfigModule, #[import] ServiceModule<'a>);

    init! {
        let provider: AppProvider<'_> = ConfigModule, ServiceModule;
    }

    let svc: Service = provider.provide();
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
    struct NumModule;

    #[injectable]
    #[module]
    #[export(String, |x: i32| format!("n={}", x))]
    struct FmtModule;

    #[injectable]
    #[module]
    #[export(u64, 999)]
    struct IdModule;

    #[injectable]
    #[provider]
    struct FmtProvider(#[import] NumModule, #[import] FmtModule);

    #[injectable]
    #[provider]
    struct IdProvider(#[import] IdModule);

    init! {
        let fmt_prov: FmtProvider = NumModule, FmtModule;
        let id_prov: IdProvider = IdModule;
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
    struct BaseModule;

    #[injectable]
    #[module]
    #[export(u64, |x: i32| x as u64 * 3)]
    struct MiddleModule;

    #[injectable]
    #[module]
    #[export(String, |x: u64| format!("final={}", x))]
    struct TopModule;

    #[injectable]
    #[provider]
    struct FullProvider(
        #[import] BaseModule,
        #[import] MiddleModule,
        #[import] TopModule,
    );

    init! {
        let provider: FullProvider = BaseModule, MiddleModule, TopModule;
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
    struct Config(#[inject(7)] i32);

    #[injectable]
    #[module]
    struct ConfigModule {
        #[export]
        cfg: Config,
    }

    #[derive(Debug, PartialEq)]
    #[injectable]
    struct Cache<'a>(#[inject(99)] i32, &'a Config);

    #[injectable]
    #[module]
    struct CacheModule<'a> {
        #[export]
        cache: Cache<'a>,
    }

    #[derive(Debug, PartialEq)]
    #[injectable]
    struct Handler<'a>(&'a Config, &'a Cache<'a>);

    #[injectable]
    #[provider]
    struct AppProvider<'a>(#[import] ConfigModule, #[import] CacheModule<'a>);

    init! {
        let provider: AppProvider<'_> = ConfigModule, CacheModule;
    }

    let cfg: &Config = provider.provide();
    let cache: &Cache = provider.provide();
    let handler: Handler = provider.provide();
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
