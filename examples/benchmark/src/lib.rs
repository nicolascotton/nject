#![cfg_attr(test, feature(test))]

#[cfg(test)]
mod tests {
    extern crate test;
    use nject::{inject, injectable, module, provider};
    use test::Bencher;

    trait DepTrait1 {}
    trait DepTrait2 {}
    trait DepTrait3 {}
    trait DepTrait4 {}
    trait DepTrait5 {}
    trait DepTrait6 {}
    trait DepTrait7 {}
    trait DepTrait8 {}
    trait DepTrait9 {}
    trait DepTrait10 {}

    #[inject(Self(123))]
    struct Dep1(i32);
    impl DepTrait1 for Dep1 {}

    #[injectable]
    struct Dep2(Dep1);
    impl DepTrait2 for Dep2 {}

    #[injectable]
    struct Dep3(Dep1, Dep2);
    impl DepTrait3 for Dep3 {}

    #[injectable]
    struct Dep4(Dep1, Dep2, Dep3);
    impl DepTrait4 for Dep4 {}

    #[injectable]
    struct Dep5(Dep1, Dep2, Dep3, Dep4);
    impl DepTrait5 for Dep5 {}

    #[injectable]
    struct Dep6(Dep1, Dep2, Dep3, Dep4, Dep5);
    impl DepTrait6 for Dep6 {}

    #[injectable]
    struct Dep7(Dep1, Dep2, Dep3, Dep4, Dep5, Dep6);
    impl DepTrait7 for Dep7 {}

    #[injectable]
    struct Dep8(Dep1, Dep2, Dep3, Dep4, Dep5, Dep6, Dep7);
    impl DepTrait8 for Dep8 {}

    #[injectable]
    struct Dep9(Dep1, Dep2, Dep3, Dep4, Dep5, Dep6, Dep7, Dep8);
    impl DepTrait9 for Dep9 {}

    #[injectable]
    struct Dep10(Dep1, Dep2, Dep3, Dep4, Dep5, Dep6, Dep7, Dep8, Dep9);
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

    const ITERATION_COUNT: i32 = 10000;

    #[bench]
    fn nject_by_value(b: &mut Bencher) {
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
    fn nject_by_ref(b: &mut Bencher) {
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
    fn nject_by_ref_dyn(b: &mut Bencher) {
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
    fn nject_by_module_ref(b: &mut Bencher) {
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
    fn nject_by_module_ref_dyn(b: &mut Bencher) {
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
    fn baseline_by_value(b: &mut Bencher) {
        b.iter(|| {
            for _ in 0..ITERATION_COUNT {
                test::black_box((
                    Dep1(123),
                    Dep2(Dep1(123)),
                    Dep3(Dep1(123), Dep2(Dep1(123))),
                    Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                    Dep5(
                        Dep1(123),
                        Dep2(Dep1(123)),
                        Dep3(Dep1(123), Dep2(Dep1(123))),
                        Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                    ),
                    Dep6(
                        Dep1(123),
                        Dep2(Dep1(123)),
                        Dep3(Dep1(123), Dep2(Dep1(123))),
                        Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                        Dep5(
                            Dep1(123),
                            Dep2(Dep1(123)),
                            Dep3(Dep1(123), Dep2(Dep1(123))),
                            Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                        ),
                    ),
                    Dep7(
                        Dep1(123),
                        Dep2(Dep1(123)),
                        Dep3(Dep1(123), Dep2(Dep1(123))),
                        Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                        Dep5(
                            Dep1(123),
                            Dep2(Dep1(123)),
                            Dep3(Dep1(123), Dep2(Dep1(123))),
                            Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                        ),
                        Dep6(
                            Dep1(123),
                            Dep2(Dep1(123)),
                            Dep3(Dep1(123), Dep2(Dep1(123))),
                            Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                            Dep5(
                                Dep1(123),
                                Dep2(Dep1(123)),
                                Dep3(Dep1(123), Dep2(Dep1(123))),
                                Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                            ),
                        ),
                    ),
                    Dep8(
                        Dep1(123),
                        Dep2(Dep1(123)),
                        Dep3(Dep1(123), Dep2(Dep1(123))),
                        Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                        Dep5(
                            Dep1(123),
                            Dep2(Dep1(123)),
                            Dep3(Dep1(123), Dep2(Dep1(123))),
                            Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                        ),
                        Dep6(
                            Dep1(123),
                            Dep2(Dep1(123)),
                            Dep3(Dep1(123), Dep2(Dep1(123))),
                            Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                            Dep5(
                                Dep1(123),
                                Dep2(Dep1(123)),
                                Dep3(Dep1(123), Dep2(Dep1(123))),
                                Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                            ),
                        ),
                        Dep7(
                            Dep1(123),
                            Dep2(Dep1(123)),
                            Dep3(Dep1(123), Dep2(Dep1(123))),
                            Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                            Dep5(
                                Dep1(123),
                                Dep2(Dep1(123)),
                                Dep3(Dep1(123), Dep2(Dep1(123))),
                                Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                            ),
                            Dep6(
                                Dep1(123),
                                Dep2(Dep1(123)),
                                Dep3(Dep1(123), Dep2(Dep1(123))),
                                Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                                Dep5(
                                    Dep1(123),
                                    Dep2(Dep1(123)),
                                    Dep3(Dep1(123), Dep2(Dep1(123))),
                                    Dep4(
                                        Dep1(123),
                                        Dep2(Dep1(123)),
                                        Dep3(Dep1(123), Dep2(Dep1(123))),
                                    ),
                                ),
                            ),
                        ),
                    ),
                    Dep9(
                        Dep1(123),
                        Dep2(Dep1(123)),
                        Dep3(Dep1(123), Dep2(Dep1(123))),
                        Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                        Dep5(
                            Dep1(123),
                            Dep2(Dep1(123)),
                            Dep3(Dep1(123), Dep2(Dep1(123))),
                            Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                        ),
                        Dep6(
                            Dep1(123),
                            Dep2(Dep1(123)),
                            Dep3(Dep1(123), Dep2(Dep1(123))),
                            Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                            Dep5(
                                Dep1(123),
                                Dep2(Dep1(123)),
                                Dep3(Dep1(123), Dep2(Dep1(123))),
                                Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                            ),
                        ),
                        Dep7(
                            Dep1(123),
                            Dep2(Dep1(123)),
                            Dep3(Dep1(123), Dep2(Dep1(123))),
                            Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                            Dep5(
                                Dep1(123),
                                Dep2(Dep1(123)),
                                Dep3(Dep1(123), Dep2(Dep1(123))),
                                Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                            ),
                            Dep6(
                                Dep1(123),
                                Dep2(Dep1(123)),
                                Dep3(Dep1(123), Dep2(Dep1(123))),
                                Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                                Dep5(
                                    Dep1(123),
                                    Dep2(Dep1(123)),
                                    Dep3(Dep1(123), Dep2(Dep1(123))),
                                    Dep4(
                                        Dep1(123),
                                        Dep2(Dep1(123)),
                                        Dep3(Dep1(123), Dep2(Dep1(123))),
                                    ),
                                ),
                            ),
                        ),
                        Dep8(
                            Dep1(123),
                            Dep2(Dep1(123)),
                            Dep3(Dep1(123), Dep2(Dep1(123))),
                            Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                            Dep5(
                                Dep1(123),
                                Dep2(Dep1(123)),
                                Dep3(Dep1(123), Dep2(Dep1(123))),
                                Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                            ),
                            Dep6(
                                Dep1(123),
                                Dep2(Dep1(123)),
                                Dep3(Dep1(123), Dep2(Dep1(123))),
                                Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                                Dep5(
                                    Dep1(123),
                                    Dep2(Dep1(123)),
                                    Dep3(Dep1(123), Dep2(Dep1(123))),
                                    Dep4(
                                        Dep1(123),
                                        Dep2(Dep1(123)),
                                        Dep3(Dep1(123), Dep2(Dep1(123))),
                                    ),
                                ),
                            ),
                            Dep7(
                                Dep1(123),
                                Dep2(Dep1(123)),
                                Dep3(Dep1(123), Dep2(Dep1(123))),
                                Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                                Dep5(
                                    Dep1(123),
                                    Dep2(Dep1(123)),
                                    Dep3(Dep1(123), Dep2(Dep1(123))),
                                    Dep4(
                                        Dep1(123),
                                        Dep2(Dep1(123)),
                                        Dep3(Dep1(123), Dep2(Dep1(123))),
                                    ),
                                ),
                                Dep6(
                                    Dep1(123),
                                    Dep2(Dep1(123)),
                                    Dep3(Dep1(123), Dep2(Dep1(123))),
                                    Dep4(
                                        Dep1(123),
                                        Dep2(Dep1(123)),
                                        Dep3(Dep1(123), Dep2(Dep1(123))),
                                    ),
                                    Dep5(
                                        Dep1(123),
                                        Dep2(Dep1(123)),
                                        Dep3(Dep1(123), Dep2(Dep1(123))),
                                        Dep4(
                                            Dep1(123),
                                            Dep2(Dep1(123)),
                                            Dep3(Dep1(123), Dep2(Dep1(123))),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                    Dep10(
                        Dep1(123),
                        Dep2(Dep1(123)),
                        Dep3(Dep1(123), Dep2(Dep1(123))),
                        Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                        Dep5(
                            Dep1(123),
                            Dep2(Dep1(123)),
                            Dep3(Dep1(123), Dep2(Dep1(123))),
                            Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                        ),
                        Dep6(
                            Dep1(123),
                            Dep2(Dep1(123)),
                            Dep3(Dep1(123), Dep2(Dep1(123))),
                            Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                            Dep5(
                                Dep1(123),
                                Dep2(Dep1(123)),
                                Dep3(Dep1(123), Dep2(Dep1(123))),
                                Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                            ),
                        ),
                        Dep7(
                            Dep1(123),
                            Dep2(Dep1(123)),
                            Dep3(Dep1(123), Dep2(Dep1(123))),
                            Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                            Dep5(
                                Dep1(123),
                                Dep2(Dep1(123)),
                                Dep3(Dep1(123), Dep2(Dep1(123))),
                                Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                            ),
                            Dep6(
                                Dep1(123),
                                Dep2(Dep1(123)),
                                Dep3(Dep1(123), Dep2(Dep1(123))),
                                Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                                Dep5(
                                    Dep1(123),
                                    Dep2(Dep1(123)),
                                    Dep3(Dep1(123), Dep2(Dep1(123))),
                                    Dep4(
                                        Dep1(123),
                                        Dep2(Dep1(123)),
                                        Dep3(Dep1(123), Dep2(Dep1(123))),
                                    ),
                                ),
                            ),
                        ),
                        Dep8(
                            Dep1(123),
                            Dep2(Dep1(123)),
                            Dep3(Dep1(123), Dep2(Dep1(123))),
                            Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                            Dep5(
                                Dep1(123),
                                Dep2(Dep1(123)),
                                Dep3(Dep1(123), Dep2(Dep1(123))),
                                Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                            ),
                            Dep6(
                                Dep1(123),
                                Dep2(Dep1(123)),
                                Dep3(Dep1(123), Dep2(Dep1(123))),
                                Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                                Dep5(
                                    Dep1(123),
                                    Dep2(Dep1(123)),
                                    Dep3(Dep1(123), Dep2(Dep1(123))),
                                    Dep4(
                                        Dep1(123),
                                        Dep2(Dep1(123)),
                                        Dep3(Dep1(123), Dep2(Dep1(123))),
                                    ),
                                ),
                            ),
                            Dep7(
                                Dep1(123),
                                Dep2(Dep1(123)),
                                Dep3(Dep1(123), Dep2(Dep1(123))),
                                Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                                Dep5(
                                    Dep1(123),
                                    Dep2(Dep1(123)),
                                    Dep3(Dep1(123), Dep2(Dep1(123))),
                                    Dep4(
                                        Dep1(123),
                                        Dep2(Dep1(123)),
                                        Dep3(Dep1(123), Dep2(Dep1(123))),
                                    ),
                                ),
                                Dep6(
                                    Dep1(123),
                                    Dep2(Dep1(123)),
                                    Dep3(Dep1(123), Dep2(Dep1(123))),
                                    Dep4(
                                        Dep1(123),
                                        Dep2(Dep1(123)),
                                        Dep3(Dep1(123), Dep2(Dep1(123))),
                                    ),
                                    Dep5(
                                        Dep1(123),
                                        Dep2(Dep1(123)),
                                        Dep3(Dep1(123), Dep2(Dep1(123))),
                                        Dep4(
                                            Dep1(123),
                                            Dep2(Dep1(123)),
                                            Dep3(Dep1(123), Dep2(Dep1(123))),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                        Dep9(
                            Dep1(123),
                            Dep2(Dep1(123)),
                            Dep3(Dep1(123), Dep2(Dep1(123))),
                            Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                            Dep5(
                                Dep1(123),
                                Dep2(Dep1(123)),
                                Dep3(Dep1(123), Dep2(Dep1(123))),
                                Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                            ),
                            Dep6(
                                Dep1(123),
                                Dep2(Dep1(123)),
                                Dep3(Dep1(123), Dep2(Dep1(123))),
                                Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                                Dep5(
                                    Dep1(123),
                                    Dep2(Dep1(123)),
                                    Dep3(Dep1(123), Dep2(Dep1(123))),
                                    Dep4(
                                        Dep1(123),
                                        Dep2(Dep1(123)),
                                        Dep3(Dep1(123), Dep2(Dep1(123))),
                                    ),
                                ),
                            ),
                            Dep7(
                                Dep1(123),
                                Dep2(Dep1(123)),
                                Dep3(Dep1(123), Dep2(Dep1(123))),
                                Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                                Dep5(
                                    Dep1(123),
                                    Dep2(Dep1(123)),
                                    Dep3(Dep1(123), Dep2(Dep1(123))),
                                    Dep4(
                                        Dep1(123),
                                        Dep2(Dep1(123)),
                                        Dep3(Dep1(123), Dep2(Dep1(123))),
                                    ),
                                ),
                                Dep6(
                                    Dep1(123),
                                    Dep2(Dep1(123)),
                                    Dep3(Dep1(123), Dep2(Dep1(123))),
                                    Dep4(
                                        Dep1(123),
                                        Dep2(Dep1(123)),
                                        Dep3(Dep1(123), Dep2(Dep1(123))),
                                    ),
                                    Dep5(
                                        Dep1(123),
                                        Dep2(Dep1(123)),
                                        Dep3(Dep1(123), Dep2(Dep1(123))),
                                        Dep4(
                                            Dep1(123),
                                            Dep2(Dep1(123)),
                                            Dep3(Dep1(123), Dep2(Dep1(123))),
                                        ),
                                    ),
                                ),
                            ),
                            Dep8(
                                Dep1(123),
                                Dep2(Dep1(123)),
                                Dep3(Dep1(123), Dep2(Dep1(123))),
                                Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                                Dep5(
                                    Dep1(123),
                                    Dep2(Dep1(123)),
                                    Dep3(Dep1(123), Dep2(Dep1(123))),
                                    Dep4(
                                        Dep1(123),
                                        Dep2(Dep1(123)),
                                        Dep3(Dep1(123), Dep2(Dep1(123))),
                                    ),
                                ),
                                Dep6(
                                    Dep1(123),
                                    Dep2(Dep1(123)),
                                    Dep3(Dep1(123), Dep2(Dep1(123))),
                                    Dep4(
                                        Dep1(123),
                                        Dep2(Dep1(123)),
                                        Dep3(Dep1(123), Dep2(Dep1(123))),
                                    ),
                                    Dep5(
                                        Dep1(123),
                                        Dep2(Dep1(123)),
                                        Dep3(Dep1(123), Dep2(Dep1(123))),
                                        Dep4(
                                            Dep1(123),
                                            Dep2(Dep1(123)),
                                            Dep3(Dep1(123), Dep2(Dep1(123))),
                                        ),
                                    ),
                                ),
                                Dep7(
                                    Dep1(123),
                                    Dep2(Dep1(123)),
                                    Dep3(Dep1(123), Dep2(Dep1(123))),
                                    Dep4(
                                        Dep1(123),
                                        Dep2(Dep1(123)),
                                        Dep3(Dep1(123), Dep2(Dep1(123))),
                                    ),
                                    Dep5(
                                        Dep1(123),
                                        Dep2(Dep1(123)),
                                        Dep3(Dep1(123), Dep2(Dep1(123))),
                                        Dep4(
                                            Dep1(123),
                                            Dep2(Dep1(123)),
                                            Dep3(Dep1(123), Dep2(Dep1(123))),
                                        ),
                                    ),
                                    Dep6(
                                        Dep1(123),
                                        Dep2(Dep1(123)),
                                        Dep3(Dep1(123), Dep2(Dep1(123))),
                                        Dep4(
                                            Dep1(123),
                                            Dep2(Dep1(123)),
                                            Dep3(Dep1(123), Dep2(Dep1(123))),
                                        ),
                                        Dep5(
                                            Dep1(123),
                                            Dep2(Dep1(123)),
                                            Dep3(Dep1(123), Dep2(Dep1(123))),
                                            Dep4(
                                                Dep1(123),
                                                Dep2(Dep1(123)),
                                                Dep3(Dep1(123), Dep2(Dep1(123))),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ));
            }
        });
    }

    #[bench]
    fn baseline_by_ref(b: &mut Bencher) {
        let provider = (
            Dep1(123),
            Dep2(Dep1(123)),
            Dep3(Dep1(123), Dep2(Dep1(123))),
            Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
            Dep5(
                Dep1(123),
                Dep2(Dep1(123)),
                Dep3(Dep1(123), Dep2(Dep1(123))),
                Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
            ),
            Dep6(
                Dep1(123),
                Dep2(Dep1(123)),
                Dep3(Dep1(123), Dep2(Dep1(123))),
                Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                Dep5(
                    Dep1(123),
                    Dep2(Dep1(123)),
                    Dep3(Dep1(123), Dep2(Dep1(123))),
                    Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                ),
            ),
            Dep7(
                Dep1(123),
                Dep2(Dep1(123)),
                Dep3(Dep1(123), Dep2(Dep1(123))),
                Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                Dep5(
                    Dep1(123),
                    Dep2(Dep1(123)),
                    Dep3(Dep1(123), Dep2(Dep1(123))),
                    Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                ),
                Dep6(
                    Dep1(123),
                    Dep2(Dep1(123)),
                    Dep3(Dep1(123), Dep2(Dep1(123))),
                    Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                    Dep5(
                        Dep1(123),
                        Dep2(Dep1(123)),
                        Dep3(Dep1(123), Dep2(Dep1(123))),
                        Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                    ),
                ),
            ),
            Dep8(
                Dep1(123),
                Dep2(Dep1(123)),
                Dep3(Dep1(123), Dep2(Dep1(123))),
                Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                Dep5(
                    Dep1(123),
                    Dep2(Dep1(123)),
                    Dep3(Dep1(123), Dep2(Dep1(123))),
                    Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                ),
                Dep6(
                    Dep1(123),
                    Dep2(Dep1(123)),
                    Dep3(Dep1(123), Dep2(Dep1(123))),
                    Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                    Dep5(
                        Dep1(123),
                        Dep2(Dep1(123)),
                        Dep3(Dep1(123), Dep2(Dep1(123))),
                        Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                    ),
                ),
                Dep7(
                    Dep1(123),
                    Dep2(Dep1(123)),
                    Dep3(Dep1(123), Dep2(Dep1(123))),
                    Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                    Dep5(
                        Dep1(123),
                        Dep2(Dep1(123)),
                        Dep3(Dep1(123), Dep2(Dep1(123))),
                        Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                    ),
                    Dep6(
                        Dep1(123),
                        Dep2(Dep1(123)),
                        Dep3(Dep1(123), Dep2(Dep1(123))),
                        Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                        Dep5(
                            Dep1(123),
                            Dep2(Dep1(123)),
                            Dep3(Dep1(123), Dep2(Dep1(123))),
                            Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                        ),
                    ),
                ),
            ),
            Dep9(
                Dep1(123),
                Dep2(Dep1(123)),
                Dep3(Dep1(123), Dep2(Dep1(123))),
                Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                Dep5(
                    Dep1(123),
                    Dep2(Dep1(123)),
                    Dep3(Dep1(123), Dep2(Dep1(123))),
                    Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                ),
                Dep6(
                    Dep1(123),
                    Dep2(Dep1(123)),
                    Dep3(Dep1(123), Dep2(Dep1(123))),
                    Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                    Dep5(
                        Dep1(123),
                        Dep2(Dep1(123)),
                        Dep3(Dep1(123), Dep2(Dep1(123))),
                        Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                    ),
                ),
                Dep7(
                    Dep1(123),
                    Dep2(Dep1(123)),
                    Dep3(Dep1(123), Dep2(Dep1(123))),
                    Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                    Dep5(
                        Dep1(123),
                        Dep2(Dep1(123)),
                        Dep3(Dep1(123), Dep2(Dep1(123))),
                        Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                    ),
                    Dep6(
                        Dep1(123),
                        Dep2(Dep1(123)),
                        Dep3(Dep1(123), Dep2(Dep1(123))),
                        Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                        Dep5(
                            Dep1(123),
                            Dep2(Dep1(123)),
                            Dep3(Dep1(123), Dep2(Dep1(123))),
                            Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                        ),
                    ),
                ),
                Dep8(
                    Dep1(123),
                    Dep2(Dep1(123)),
                    Dep3(Dep1(123), Dep2(Dep1(123))),
                    Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                    Dep5(
                        Dep1(123),
                        Dep2(Dep1(123)),
                        Dep3(Dep1(123), Dep2(Dep1(123))),
                        Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                    ),
                    Dep6(
                        Dep1(123),
                        Dep2(Dep1(123)),
                        Dep3(Dep1(123), Dep2(Dep1(123))),
                        Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                        Dep5(
                            Dep1(123),
                            Dep2(Dep1(123)),
                            Dep3(Dep1(123), Dep2(Dep1(123))),
                            Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                        ),
                    ),
                    Dep7(
                        Dep1(123),
                        Dep2(Dep1(123)),
                        Dep3(Dep1(123), Dep2(Dep1(123))),
                        Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                        Dep5(
                            Dep1(123),
                            Dep2(Dep1(123)),
                            Dep3(Dep1(123), Dep2(Dep1(123))),
                            Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                        ),
                        Dep6(
                            Dep1(123),
                            Dep2(Dep1(123)),
                            Dep3(Dep1(123), Dep2(Dep1(123))),
                            Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                            Dep5(
                                Dep1(123),
                                Dep2(Dep1(123)),
                                Dep3(Dep1(123), Dep2(Dep1(123))),
                                Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                            ),
                        ),
                    ),
                ),
            ),
            Dep10(
                Dep1(123),
                Dep2(Dep1(123)),
                Dep3(Dep1(123), Dep2(Dep1(123))),
                Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                Dep5(
                    Dep1(123),
                    Dep2(Dep1(123)),
                    Dep3(Dep1(123), Dep2(Dep1(123))),
                    Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                ),
                Dep6(
                    Dep1(123),
                    Dep2(Dep1(123)),
                    Dep3(Dep1(123), Dep2(Dep1(123))),
                    Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                    Dep5(
                        Dep1(123),
                        Dep2(Dep1(123)),
                        Dep3(Dep1(123), Dep2(Dep1(123))),
                        Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                    ),
                ),
                Dep7(
                    Dep1(123),
                    Dep2(Dep1(123)),
                    Dep3(Dep1(123), Dep2(Dep1(123))),
                    Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                    Dep5(
                        Dep1(123),
                        Dep2(Dep1(123)),
                        Dep3(Dep1(123), Dep2(Dep1(123))),
                        Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                    ),
                    Dep6(
                        Dep1(123),
                        Dep2(Dep1(123)),
                        Dep3(Dep1(123), Dep2(Dep1(123))),
                        Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                        Dep5(
                            Dep1(123),
                            Dep2(Dep1(123)),
                            Dep3(Dep1(123), Dep2(Dep1(123))),
                            Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                        ),
                    ),
                ),
                Dep8(
                    Dep1(123),
                    Dep2(Dep1(123)),
                    Dep3(Dep1(123), Dep2(Dep1(123))),
                    Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                    Dep5(
                        Dep1(123),
                        Dep2(Dep1(123)),
                        Dep3(Dep1(123), Dep2(Dep1(123))),
                        Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                    ),
                    Dep6(
                        Dep1(123),
                        Dep2(Dep1(123)),
                        Dep3(Dep1(123), Dep2(Dep1(123))),
                        Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                        Dep5(
                            Dep1(123),
                            Dep2(Dep1(123)),
                            Dep3(Dep1(123), Dep2(Dep1(123))),
                            Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                        ),
                    ),
                    Dep7(
                        Dep1(123),
                        Dep2(Dep1(123)),
                        Dep3(Dep1(123), Dep2(Dep1(123))),
                        Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                        Dep5(
                            Dep1(123),
                            Dep2(Dep1(123)),
                            Dep3(Dep1(123), Dep2(Dep1(123))),
                            Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                        ),
                        Dep6(
                            Dep1(123),
                            Dep2(Dep1(123)),
                            Dep3(Dep1(123), Dep2(Dep1(123))),
                            Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                            Dep5(
                                Dep1(123),
                                Dep2(Dep1(123)),
                                Dep3(Dep1(123), Dep2(Dep1(123))),
                                Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                            ),
                        ),
                    ),
                ),
                Dep9(
                    Dep1(123),
                    Dep2(Dep1(123)),
                    Dep3(Dep1(123), Dep2(Dep1(123))),
                    Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                    Dep5(
                        Dep1(123),
                        Dep2(Dep1(123)),
                        Dep3(Dep1(123), Dep2(Dep1(123))),
                        Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                    ),
                    Dep6(
                        Dep1(123),
                        Dep2(Dep1(123)),
                        Dep3(Dep1(123), Dep2(Dep1(123))),
                        Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                        Dep5(
                            Dep1(123),
                            Dep2(Dep1(123)),
                            Dep3(Dep1(123), Dep2(Dep1(123))),
                            Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                        ),
                    ),
                    Dep7(
                        Dep1(123),
                        Dep2(Dep1(123)),
                        Dep3(Dep1(123), Dep2(Dep1(123))),
                        Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                        Dep5(
                            Dep1(123),
                            Dep2(Dep1(123)),
                            Dep3(Dep1(123), Dep2(Dep1(123))),
                            Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                        ),
                        Dep6(
                            Dep1(123),
                            Dep2(Dep1(123)),
                            Dep3(Dep1(123), Dep2(Dep1(123))),
                            Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                            Dep5(
                                Dep1(123),
                                Dep2(Dep1(123)),
                                Dep3(Dep1(123), Dep2(Dep1(123))),
                                Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                            ),
                        ),
                    ),
                    Dep8(
                        Dep1(123),
                        Dep2(Dep1(123)),
                        Dep3(Dep1(123), Dep2(Dep1(123))),
                        Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                        Dep5(
                            Dep1(123),
                            Dep2(Dep1(123)),
                            Dep3(Dep1(123), Dep2(Dep1(123))),
                            Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                        ),
                        Dep6(
                            Dep1(123),
                            Dep2(Dep1(123)),
                            Dep3(Dep1(123), Dep2(Dep1(123))),
                            Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                            Dep5(
                                Dep1(123),
                                Dep2(Dep1(123)),
                                Dep3(Dep1(123), Dep2(Dep1(123))),
                                Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                            ),
                        ),
                        Dep7(
                            Dep1(123),
                            Dep2(Dep1(123)),
                            Dep3(Dep1(123), Dep2(Dep1(123))),
                            Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                            Dep5(
                                Dep1(123),
                                Dep2(Dep1(123)),
                                Dep3(Dep1(123), Dep2(Dep1(123))),
                                Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                            ),
                            Dep6(
                                Dep1(123),
                                Dep2(Dep1(123)),
                                Dep3(Dep1(123), Dep2(Dep1(123))),
                                Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                                Dep5(
                                    Dep1(123),
                                    Dep2(Dep1(123)),
                                    Dep3(Dep1(123), Dep2(Dep1(123))),
                                    Dep4(
                                        Dep1(123),
                                        Dep2(Dep1(123)),
                                        Dep3(Dep1(123), Dep2(Dep1(123))),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        );
        b.iter(|| {
            for _ in 0..ITERATION_COUNT {
                test::black_box((
                    &provider.0,
                    &provider.1,
                    &provider.2,
                    &provider.3,
                    &provider.4,
                    &provider.5,
                    &provider.6,
                    &provider.7,
                    &provider.8,
                    &provider.9,
                ));
            }
        });
    }

    #[bench]
    fn baseline_by_ref_dyn(b: &mut Bencher) {
        let provider = (
            Dep1(123),
            Dep2(Dep1(123)),
            Dep3(Dep1(123), Dep2(Dep1(123))),
            Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
            Dep5(
                Dep1(123),
                Dep2(Dep1(123)),
                Dep3(Dep1(123), Dep2(Dep1(123))),
                Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
            ),
            Dep6(
                Dep1(123),
                Dep2(Dep1(123)),
                Dep3(Dep1(123), Dep2(Dep1(123))),
                Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                Dep5(
                    Dep1(123),
                    Dep2(Dep1(123)),
                    Dep3(Dep1(123), Dep2(Dep1(123))),
                    Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                ),
            ),
            Dep7(
                Dep1(123),
                Dep2(Dep1(123)),
                Dep3(Dep1(123), Dep2(Dep1(123))),
                Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                Dep5(
                    Dep1(123),
                    Dep2(Dep1(123)),
                    Dep3(Dep1(123), Dep2(Dep1(123))),
                    Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                ),
                Dep6(
                    Dep1(123),
                    Dep2(Dep1(123)),
                    Dep3(Dep1(123), Dep2(Dep1(123))),
                    Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                    Dep5(
                        Dep1(123),
                        Dep2(Dep1(123)),
                        Dep3(Dep1(123), Dep2(Dep1(123))),
                        Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                    ),
                ),
            ),
            Dep8(
                Dep1(123),
                Dep2(Dep1(123)),
                Dep3(Dep1(123), Dep2(Dep1(123))),
                Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                Dep5(
                    Dep1(123),
                    Dep2(Dep1(123)),
                    Dep3(Dep1(123), Dep2(Dep1(123))),
                    Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                ),
                Dep6(
                    Dep1(123),
                    Dep2(Dep1(123)),
                    Dep3(Dep1(123), Dep2(Dep1(123))),
                    Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                    Dep5(
                        Dep1(123),
                        Dep2(Dep1(123)),
                        Dep3(Dep1(123), Dep2(Dep1(123))),
                        Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                    ),
                ),
                Dep7(
                    Dep1(123),
                    Dep2(Dep1(123)),
                    Dep3(Dep1(123), Dep2(Dep1(123))),
                    Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                    Dep5(
                        Dep1(123),
                        Dep2(Dep1(123)),
                        Dep3(Dep1(123), Dep2(Dep1(123))),
                        Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                    ),
                    Dep6(
                        Dep1(123),
                        Dep2(Dep1(123)),
                        Dep3(Dep1(123), Dep2(Dep1(123))),
                        Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                        Dep5(
                            Dep1(123),
                            Dep2(Dep1(123)),
                            Dep3(Dep1(123), Dep2(Dep1(123))),
                            Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                        ),
                    ),
                ),
            ),
            Dep9(
                Dep1(123),
                Dep2(Dep1(123)),
                Dep3(Dep1(123), Dep2(Dep1(123))),
                Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                Dep5(
                    Dep1(123),
                    Dep2(Dep1(123)),
                    Dep3(Dep1(123), Dep2(Dep1(123))),
                    Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                ),
                Dep6(
                    Dep1(123),
                    Dep2(Dep1(123)),
                    Dep3(Dep1(123), Dep2(Dep1(123))),
                    Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                    Dep5(
                        Dep1(123),
                        Dep2(Dep1(123)),
                        Dep3(Dep1(123), Dep2(Dep1(123))),
                        Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                    ),
                ),
                Dep7(
                    Dep1(123),
                    Dep2(Dep1(123)),
                    Dep3(Dep1(123), Dep2(Dep1(123))),
                    Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                    Dep5(
                        Dep1(123),
                        Dep2(Dep1(123)),
                        Dep3(Dep1(123), Dep2(Dep1(123))),
                        Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                    ),
                    Dep6(
                        Dep1(123),
                        Dep2(Dep1(123)),
                        Dep3(Dep1(123), Dep2(Dep1(123))),
                        Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                        Dep5(
                            Dep1(123),
                            Dep2(Dep1(123)),
                            Dep3(Dep1(123), Dep2(Dep1(123))),
                            Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                        ),
                    ),
                ),
                Dep8(
                    Dep1(123),
                    Dep2(Dep1(123)),
                    Dep3(Dep1(123), Dep2(Dep1(123))),
                    Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                    Dep5(
                        Dep1(123),
                        Dep2(Dep1(123)),
                        Dep3(Dep1(123), Dep2(Dep1(123))),
                        Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                    ),
                    Dep6(
                        Dep1(123),
                        Dep2(Dep1(123)),
                        Dep3(Dep1(123), Dep2(Dep1(123))),
                        Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                        Dep5(
                            Dep1(123),
                            Dep2(Dep1(123)),
                            Dep3(Dep1(123), Dep2(Dep1(123))),
                            Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                        ),
                    ),
                    Dep7(
                        Dep1(123),
                        Dep2(Dep1(123)),
                        Dep3(Dep1(123), Dep2(Dep1(123))),
                        Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                        Dep5(
                            Dep1(123),
                            Dep2(Dep1(123)),
                            Dep3(Dep1(123), Dep2(Dep1(123))),
                            Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                        ),
                        Dep6(
                            Dep1(123),
                            Dep2(Dep1(123)),
                            Dep3(Dep1(123), Dep2(Dep1(123))),
                            Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                            Dep5(
                                Dep1(123),
                                Dep2(Dep1(123)),
                                Dep3(Dep1(123), Dep2(Dep1(123))),
                                Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                            ),
                        ),
                    ),
                ),
            ),
            Dep10(
                Dep1(123),
                Dep2(Dep1(123)),
                Dep3(Dep1(123), Dep2(Dep1(123))),
                Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                Dep5(
                    Dep1(123),
                    Dep2(Dep1(123)),
                    Dep3(Dep1(123), Dep2(Dep1(123))),
                    Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                ),
                Dep6(
                    Dep1(123),
                    Dep2(Dep1(123)),
                    Dep3(Dep1(123), Dep2(Dep1(123))),
                    Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                    Dep5(
                        Dep1(123),
                        Dep2(Dep1(123)),
                        Dep3(Dep1(123), Dep2(Dep1(123))),
                        Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                    ),
                ),
                Dep7(
                    Dep1(123),
                    Dep2(Dep1(123)),
                    Dep3(Dep1(123), Dep2(Dep1(123))),
                    Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                    Dep5(
                        Dep1(123),
                        Dep2(Dep1(123)),
                        Dep3(Dep1(123), Dep2(Dep1(123))),
                        Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                    ),
                    Dep6(
                        Dep1(123),
                        Dep2(Dep1(123)),
                        Dep3(Dep1(123), Dep2(Dep1(123))),
                        Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                        Dep5(
                            Dep1(123),
                            Dep2(Dep1(123)),
                            Dep3(Dep1(123), Dep2(Dep1(123))),
                            Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                        ),
                    ),
                ),
                Dep8(
                    Dep1(123),
                    Dep2(Dep1(123)),
                    Dep3(Dep1(123), Dep2(Dep1(123))),
                    Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                    Dep5(
                        Dep1(123),
                        Dep2(Dep1(123)),
                        Dep3(Dep1(123), Dep2(Dep1(123))),
                        Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                    ),
                    Dep6(
                        Dep1(123),
                        Dep2(Dep1(123)),
                        Dep3(Dep1(123), Dep2(Dep1(123))),
                        Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                        Dep5(
                            Dep1(123),
                            Dep2(Dep1(123)),
                            Dep3(Dep1(123), Dep2(Dep1(123))),
                            Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                        ),
                    ),
                    Dep7(
                        Dep1(123),
                        Dep2(Dep1(123)),
                        Dep3(Dep1(123), Dep2(Dep1(123))),
                        Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                        Dep5(
                            Dep1(123),
                            Dep2(Dep1(123)),
                            Dep3(Dep1(123), Dep2(Dep1(123))),
                            Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                        ),
                        Dep6(
                            Dep1(123),
                            Dep2(Dep1(123)),
                            Dep3(Dep1(123), Dep2(Dep1(123))),
                            Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                            Dep5(
                                Dep1(123),
                                Dep2(Dep1(123)),
                                Dep3(Dep1(123), Dep2(Dep1(123))),
                                Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                            ),
                        ),
                    ),
                ),
                Dep9(
                    Dep1(123),
                    Dep2(Dep1(123)),
                    Dep3(Dep1(123), Dep2(Dep1(123))),
                    Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                    Dep5(
                        Dep1(123),
                        Dep2(Dep1(123)),
                        Dep3(Dep1(123), Dep2(Dep1(123))),
                        Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                    ),
                    Dep6(
                        Dep1(123),
                        Dep2(Dep1(123)),
                        Dep3(Dep1(123), Dep2(Dep1(123))),
                        Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                        Dep5(
                            Dep1(123),
                            Dep2(Dep1(123)),
                            Dep3(Dep1(123), Dep2(Dep1(123))),
                            Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                        ),
                    ),
                    Dep7(
                        Dep1(123),
                        Dep2(Dep1(123)),
                        Dep3(Dep1(123), Dep2(Dep1(123))),
                        Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                        Dep5(
                            Dep1(123),
                            Dep2(Dep1(123)),
                            Dep3(Dep1(123), Dep2(Dep1(123))),
                            Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                        ),
                        Dep6(
                            Dep1(123),
                            Dep2(Dep1(123)),
                            Dep3(Dep1(123), Dep2(Dep1(123))),
                            Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                            Dep5(
                                Dep1(123),
                                Dep2(Dep1(123)),
                                Dep3(Dep1(123), Dep2(Dep1(123))),
                                Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                            ),
                        ),
                    ),
                    Dep8(
                        Dep1(123),
                        Dep2(Dep1(123)),
                        Dep3(Dep1(123), Dep2(Dep1(123))),
                        Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                        Dep5(
                            Dep1(123),
                            Dep2(Dep1(123)),
                            Dep3(Dep1(123), Dep2(Dep1(123))),
                            Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                        ),
                        Dep6(
                            Dep1(123),
                            Dep2(Dep1(123)),
                            Dep3(Dep1(123), Dep2(Dep1(123))),
                            Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                            Dep5(
                                Dep1(123),
                                Dep2(Dep1(123)),
                                Dep3(Dep1(123), Dep2(Dep1(123))),
                                Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                            ),
                        ),
                        Dep7(
                            Dep1(123),
                            Dep2(Dep1(123)),
                            Dep3(Dep1(123), Dep2(Dep1(123))),
                            Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                            Dep5(
                                Dep1(123),
                                Dep2(Dep1(123)),
                                Dep3(Dep1(123), Dep2(Dep1(123))),
                                Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                            ),
                            Dep6(
                                Dep1(123),
                                Dep2(Dep1(123)),
                                Dep3(Dep1(123), Dep2(Dep1(123))),
                                Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                                Dep5(
                                    Dep1(123),
                                    Dep2(Dep1(123)),
                                    Dep3(Dep1(123), Dep2(Dep1(123))),
                                    Dep4(
                                        Dep1(123),
                                        Dep2(Dep1(123)),
                                        Dep3(Dep1(123), Dep2(Dep1(123))),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        );
        b.iter(|| {
            for _ in 0..ITERATION_COUNT {
                let one: &dyn DepTrait1 = &provider.0;
                let two: &dyn DepTrait2 = &provider.1;
                let three: &dyn DepTrait3 = &provider.2;
                let four: &dyn DepTrait4 = &provider.3;
                let five: &dyn DepTrait5 = &provider.4;
                let six: &dyn DepTrait6 = &provider.5;
                let seven: &dyn DepTrait7 = &provider.6;
                let eight: &dyn DepTrait8 = &provider.7;
                let nine: &dyn DepTrait9 = &provider.8;
                let ten: &dyn DepTrait10 = &provider.9;
                test::black_box((one, two, three, four, five, six, seven, eight, nine, ten));
            }
        });
    }
}
