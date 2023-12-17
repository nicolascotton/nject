use super::*;
use nject::{injectable, provider};
use test::Bencher;

#[bench]
fn by_scope_from_scope(b: &mut Bencher) {
    #[provider]
    #[scope(Dep1)]
    #[scope(Dep2)]
    #[scope(Dep3)]
    #[scope(Dep4)]
    #[scope(Dep5)]
    #[scope(Dep6)]
    #[scope(Dep7)]
    #[scope(Dep8)]
    #[scope(Dep9)]
    #[scope(Dep10)]
    struct Root;

    b.iter(move || {
        for _ in 0..ITERATION_COUNT {
            let scope = Root.scope();
            test::black_box((scope.provide::<&Dep1>(), scope.provide::<&Dep10>()));
        }
    });
}

#[bench]
fn by_scope_from_root(b: &mut Bencher) {
    #[injectable]
    struct Something;

    #[injectable]
    struct Scoped<'a>(
        &'a Dep1,
        &'a Dep2,
        &'a Dep3,
        &'a Dep4,
        &'a Dep5,
        &'a Dep6,
        &'a Dep7,
        &'a Dep8,
        &'a Dep9,
        &'a Dep10,
    );

    #[provider]
    #[injectable]
    #[scope(Something)]
    struct Root(
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
    struct InitProvider;

    let root = InitProvider.provide::<Root>();
    b.iter(move || {
        for _ in 0..ITERATION_COUNT {
            let scope = root.scope();
            test::black_box(scope.provide::<Scoped>());
        }
    });
}
