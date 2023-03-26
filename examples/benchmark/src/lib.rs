#![cfg_attr(test, feature(test))]

#[cfg(test)]
mod tests {
    extern crate test;
    use nject::{inject, injectable, provide, provider};
    use test::Bencher;

    #[inject(Self(123))]
    struct Dep1(i32);

    #[injectable]
    struct Dep2(Dep1);

    #[injectable]
    struct Dep3(Dep1, Dep2);

    #[injectable]
    struct Dep4(Dep1, Dep2, Dep3);

    #[injectable]
    struct Dep5(Dep1, Dep2, Dep3, Dep4);

    #[injectable]
    struct Dep6(Dep1, Dep2, Dep3, Dep4, Dep5);

    #[injectable]
    struct Dep7(Dep1, Dep2, Dep3, Dep4, Dep5, Dep6);

    #[injectable]
    struct Dep8(Dep1, Dep2, Dep3, Dep4, Dep5, Dep6, Dep7);

    #[injectable]
    struct Dep9(Dep1, Dep2, Dep3, Dep4, Dep5, Dep6, Dep7, Dep8);

    #[injectable]
    struct Dep10(Dep1, Dep2, Dep3, Dep4, Dep5, Dep6, Dep7, Dep8, Dep9);

    #[provider]
    struct Provider;

    #[provider]
    #[provide(&'prov Dep1, &self.0)]
    #[provide(&'prov Dep2, &self.1)]
    #[provide(&'prov Dep3, &self.2)]
    #[provide(&'prov Dep4, &self.3)]
    #[provide(&'prov Dep5, &self.4)]
    #[provide(&'prov Dep6, &self.5)]
    #[provide(&'prov Dep7, &self.6)]
    #[provide(&'prov Dep8, &self.7)]
    #[provide(&'prov Dep9, &self.8)]
    #[provide(&'prov Dep10, &self.9)]
    #[injectable]
    struct RefProvider(Dep1, Dep2, Dep3, Dep4, Dep5, Dep6, Dep7, Dep8, Dep9, Dep10);

    #[provider]
    struct ExtendedProvider(#[extend] Provider);

    #[provider]
    #[injectable]
    struct ExtendedRefProvider(#[extend] RefProvider);

    const ITERATION_COUNT: i32 = 10000;

    #[bench]
    fn nject_by_value(b: &mut Bencher) {
        let provider = Provider;
        b.iter(move || {
            let n = test::black_box(ITERATION_COUNT);
            for _ in 0..n {
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
            let n = test::black_box(ITERATION_COUNT);
            for _ in 0..n {
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
    fn nject_extended_by_value(b: &mut Bencher) {
        let provider = ExtendedProvider(Provider);
        b.iter(move || {
            let n = test::black_box(ITERATION_COUNT);
            for _ in 0..n {
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
    fn nject_extended_by_ref(b: &mut Bencher) {
        let provider = Provider.provide::<ExtendedRefProvider>();
        b.iter(move || {
            let n = test::black_box(ITERATION_COUNT);
            for _ in 0..n {
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
    fn baseline_by_value(b: &mut Bencher) {
        b.iter(|| {
            let n = test::black_box(ITERATION_COUNT);
            for _ in 0..n {
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
            let n = test::black_box(ITERATION_COUNT);
            for _ in 0..n {
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
}
