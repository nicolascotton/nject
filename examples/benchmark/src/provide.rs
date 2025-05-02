use super::*;
use nject::{injectable, provider};
use test::Bencher;

#[provider]
struct Provider;

#[provider]
#[injectable]
struct RefProvider(
    #[provide] Dep1,
    #[provide] Dep2,
    #[provide] Dep3,
    #[provide] Dep4,
    #[provide] Dep5,
    #[provide] Dep6,
    #[provide] Dep7,
    #[provide] Dep8,
    #[provide] Dep9,
    #[provide] Dep10,
);

#[provider]
#[injectable]
struct DynProvider(
    #[provide(dyn DepTrait1)] Dep1,
    #[provide(dyn DepTrait2)] Dep2,
    #[provide(dyn DepTrait3)] Dep3,
    #[provide(dyn DepTrait4)] Dep4,
    #[provide(dyn DepTrait5)] Dep5,
    #[provide(dyn DepTrait6)] Dep6,
    #[provide(dyn DepTrait7)] Dep7,
    #[provide(dyn DepTrait8)] Dep8,
    #[provide(dyn DepTrait9)] Dep9,
    #[provide(dyn DepTrait10)] Dep10,
);

#[bench]
fn by_value(b: &mut Bencher) {
    let provider = Provider;
    b.iter(move || {
        for _ in 0..ITERATION_COUNT {
            test::black_box((
                provider.provide::<Dep1>(),
                provider.provide::<Dep2>(),
                provider.provide::<Dep3>(),
                provider.provide::<Dep4>(),
                provider.provide::<Dep5>(),
                provider.provide::<Dep6>(),
                provider.provide::<Dep7>(),
                provider.provide::<Dep8>(),
                provider.provide::<Dep9>(),
                provider.provide::<Dep10>(),
            ));
        }
    });
}

#[bench]
fn by_ref(b: &mut Bencher) {
    let provider = Provider.provide::<RefProvider>();
    b.iter(move || {
        for _ in 0..ITERATION_COUNT {
            test::black_box((
                provider.provide::<&Dep1>(),
                provider.provide::<&Dep2>(),
                provider.provide::<&Dep3>(),
                provider.provide::<&Dep4>(),
                provider.provide::<&Dep5>(),
                provider.provide::<&Dep6>(),
                provider.provide::<&Dep7>(),
                provider.provide::<&Dep8>(),
                provider.provide::<&Dep9>(),
                provider.provide::<&Dep10>(),
            ));
        }
    });
}

#[bench]
fn by_ref_dyn(b: &mut Bencher) {
    let provider = Provider.provide::<DynProvider>();
    b.iter(move || {
        for _ in 0..ITERATION_COUNT {
            test::black_box((
                provider.provide::<&dyn DepTrait1>(),
                provider.provide::<&dyn DepTrait2>(),
                provider.provide::<&dyn DepTrait3>(),
                provider.provide::<&dyn DepTrait4>(),
                provider.provide::<&dyn DepTrait5>(),
                provider.provide::<&dyn DepTrait6>(),
                provider.provide::<&dyn DepTrait7>(),
                provider.provide::<&dyn DepTrait8>(),
                provider.provide::<&dyn DepTrait9>(),
                provider.provide::<&dyn DepTrait10>(),
            ));
        }
    });
}

#[bench]
fn by_value_from_multiple(b: &mut Bencher) {
    #[provider]
    #[injectable]
    struct MultiProvider(#[import] crate::MultiExportModule);
    let provider = Provider.provide::<MultiProvider>();
    b.iter(move || {
        for _ in 0..ITERATION_COUNT {
            test::black_box((
                provider.provide::<MultiDep>(),
                provider.provide::<MultiDep>(),
                provider.provide::<MultiDep>(),
                provider.provide::<MultiDep>(),
                provider.provide::<MultiDep>(),
                provider.provide::<MultiDep>(),
                provider.provide::<MultiDep>(),
                provider.provide::<MultiDep>(),
                provider.provide::<MultiDep>(),
                provider.provide::<MultiDep>(),
            ));
        }
    });
}

#[bench]
fn by_ref_dyn_from_multiple(b: &mut Bencher) {
    #[provider]
    #[injectable]
    struct MultiProvider(#[import] crate::MultiExportModule);
    let provider = Provider.provide::<MultiProvider>();
    b.iter(move || {
        for _ in 0..ITERATION_COUNT {
            test::black_box((
                provider.provide::<&dyn MultiTrait>(),
                provider.provide::<&dyn MultiTrait>(),
                provider.provide::<&dyn MultiTrait>(),
                provider.provide::<&dyn MultiTrait>(),
                provider.provide::<&dyn MultiTrait>(),
                provider.provide::<&dyn MultiTrait>(),
                provider.provide::<&dyn MultiTrait>(),
                provider.provide::<&dyn MultiTrait>(),
                provider.provide::<&dyn MultiTrait>(),
                provider.provide::<&dyn MultiTrait>(),
            ));
        }
    });
}
