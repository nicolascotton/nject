use super::*;
use impl_test::*;
use nject::{injectable, module, provider};
use test::Bencher;

mod impl_test {
    use super::*;
    pub type ImplDepTrait1 = impl DepTrait1;
    pub type ImplDepTrait2 = impl DepTrait2;
    pub type ImplDepTrait3 = impl DepTrait3;
    pub type ImplDepTrait4 = impl DepTrait4;
    pub type ImplDepTrait5 = impl DepTrait5;
    pub type ImplDepTrait6 = impl DepTrait6;
    pub type ImplDepTrait7 = impl DepTrait7;
    pub type ImplDepTrait8 = impl DepTrait8;
    pub type ImplDepTrait9 = impl DepTrait9;
    pub type ImplDepTrait10 = impl DepTrait10;

    #[injectable]
    #[module(self::impl_test::Self)]
    #[export(&'prov ImplDepTrait1, &self.0)]
    #[export(&'prov ImplDepTrait2, &self.1)]
    #[export(&'prov ImplDepTrait3, &self.2)]
    #[export(&'prov ImplDepTrait4, &self.3)]
    #[export(&'prov ImplDepTrait5, &self.4)]
    #[export(&'prov ImplDepTrait6, &self.5)]
    #[export(&'prov ImplDepTrait7, &self.6)]
    #[export(&'prov ImplDepTrait8, &self.7)]
    #[export(&'prov ImplDepTrait9, &self.8)]
    #[export(&'prov ImplDepTrait10, &self.9)]
    pub struct ModuleImpl(Dep1, Dep2, Dep3, Dep4, Dep5, Dep6, Dep7, Dep8, Dep9, Dep10);
}

#[provider]
struct Provider;

#[injectable]
#[module]
struct Module(
    #[export] Dep1,
    #[export] Dep2,
    #[export] Dep3,
    #[export] Dep4,
    #[export] Dep5,
    #[export] Dep6,
    #[export] Dep7,
    #[export] Dep8,
    #[export] Dep9,
    #[export] Dep10,
);

#[injectable]
#[module]
struct ModuleDyn(
    #[export(dyn DepTrait1)] Dep1,
    #[export(dyn DepTrait2)] Dep2,
    #[export(dyn DepTrait3)] Dep3,
    #[export(dyn DepTrait4)] Dep4,
    #[export(dyn DepTrait5)] Dep5,
    #[export(dyn DepTrait6)] Dep6,
    #[export(dyn DepTrait7)] Dep7,
    #[export(dyn DepTrait8)] Dep8,
    #[export(dyn DepTrait9)] Dep9,
    #[export(dyn DepTrait10)] Dep10,
);

#[injectable]
#[provider]
struct ModuleProvider(#[import] Module, #[import] ModuleDyn);

#[bench]
fn by_ref_from_module(b: &mut Bencher) {
    let provider = Provider.provide::<ModuleProvider>();
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
fn by_ref_dyn_from_module(b: &mut Bencher) {
    let provider = Provider.provide::<ModuleProvider>();
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
fn by_ref_impl_from_module(b: &mut Bencher) {
    #[injectable]
    #[provider]
    struct ImplProvider(#[import] self::impl_test::ModuleImpl);

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
