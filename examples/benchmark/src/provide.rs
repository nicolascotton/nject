use super::*;
use nject::{injectable, provider};
use test::Bencher;

type ImplDepTrait1 = impl DepTrait1;
type ImplDepTrait2 = impl DepTrait2;
type ImplDepTrait3 = impl DepTrait3;
type ImplDepTrait4 = impl DepTrait4;
type ImplDepTrait5 = impl DepTrait5;
type ImplDepTrait6 = impl DepTrait6;
type ImplDepTrait7 = impl DepTrait7;
type ImplDepTrait8 = impl DepTrait8;
type ImplDepTrait9 = impl DepTrait9;
type ImplDepTrait10 = impl DepTrait10;

impl DepTrait1 for Dep1 {}
impl DepTrait2 for Dep2 {}
impl DepTrait3 for Dep3 {}
impl DepTrait4 for Dep4 {}
impl DepTrait5 for Dep5 {}
impl DepTrait6 for Dep6 {}
impl DepTrait7 for Dep7 {}
impl DepTrait8 for Dep8 {}
impl DepTrait9 for Dep9 {}
impl DepTrait10 for Dep10 {}

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

#[provider]
#[injectable]
struct ImplProvider(
    #[provide(ImplDepTrait1)] Dep1,
    #[provide(ImplDepTrait2)] Dep2,
    #[provide(ImplDepTrait3)] Dep3,
    #[provide(ImplDepTrait4)] Dep4,
    #[provide(ImplDepTrait5)] Dep5,
    #[provide(ImplDepTrait6)] Dep6,
    #[provide(ImplDepTrait7)] Dep7,
    #[provide(ImplDepTrait8)] Dep8,
    #[provide(ImplDepTrait9)] Dep9,
    #[provide(ImplDepTrait10)] Dep10,
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
fn by_ref_impl(b: &mut Bencher) {
    let provider = Provider.provide::<ImplProvider>();
    b.iter(move || {
        for _ in 0..ITERATION_COUNT {
            test::black_box((
                provider.provide::<&ImplDepTrait1>(),
                provider.provide::<&ImplDepTrait2>(),
                provider.provide::<&ImplDepTrait3>(),
                provider.provide::<&ImplDepTrait4>(),
                provider.provide::<&ImplDepTrait5>(),
                provider.provide::<&ImplDepTrait6>(),
                provider.provide::<&ImplDepTrait7>(),
                provider.provide::<&ImplDepTrait8>(),
                provider.provide::<&ImplDepTrait9>(),
                provider.provide::<&ImplDepTrait10>(),
            ));
        }
    });
}
