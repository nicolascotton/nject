use super::*;
use test::Bencher;

#[bench]
fn by_value(b: &mut Bencher) {
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
                ),
            ));
        }
    });
}

#[bench]
fn by_ref(b: &mut Bencher) {
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
                                Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
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
fn by_ref_dyn(b: &mut Bencher) {
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
                                Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
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

#[bench]
fn by_value_from_scope(b: &mut Bencher) {
    struct Root;

    let root = Root;
    b.iter(move || {
        for _ in 0..ITERATION_COUNT {
            let scope = (
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
                ),
                &root,
            );
            test::black_box((&scope.0, &scope.9));
        }
    });
}

#[bench]
fn by_ref_from_scope_root(b: &mut Bencher) {
    let root = (
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
                                Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    b.iter(move || {
        for _ in 0..ITERATION_COUNT {
            test::black_box((
                &root.0, &root.1, &root.2, &root.3, &root.4, &root.5, &root.6, &root.7, &root.8,
                &root.9,
            ));
        }
    });
}

#[bench]
fn iter_by_value(b: &mut Bencher) {
    let provider: [MultiDep; 10] = [
        MultiDep(1),
        MultiDep(2),
        MultiDep(3),
        MultiDep(4),
        MultiDep(5),
        MultiDep(6),
        MultiDep(7),
        MultiDep(8),
        MultiDep(9),
        MultiDep(10),
    ];
    b.iter(|| {
        for _ in 0..ITERATION_COUNT {
            test::black_box(for x in &provider {
                let _ = x;
            });
        }
    });
}

#[bench]
fn iter_by_dyn_ref(b: &mut Bencher) {
    let provider: [&dyn MultiTrait; 10] = [
        &Dep1(123),
        &Dep2(Dep1(123)),
        &Dep3(Dep1(123), Dep2(Dep1(123))),
        &Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
        &Dep5(
            Dep1(123),
            Dep2(Dep1(123)),
            Dep3(Dep1(123), Dep2(Dep1(123))),
            Dep4(Dep1(123), Dep2(Dep1(123)), Dep3(Dep1(123), Dep2(Dep1(123)))),
        ),
        &Dep6(
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
        &Dep7(
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
        &Dep8(
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
        &Dep9(
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
        &Dep10(
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
        ),
    ];
    b.iter(|| {
        for _ in 0..ITERATION_COUNT {
            test::black_box(for x in provider {
                let _ = x;
            });
        }
    });
}

#[bench]
fn by_value_from_multiple(b: &mut Bencher) {
    b.iter(move || {
        for _ in 0..ITERATION_COUNT {
            test::black_box((
                MultiDep(10),
                MultiDep(10),
                MultiDep(10),
                MultiDep(10),
                MultiDep(10),
                MultiDep(10),
                MultiDep(10),
                MultiDep(10),
                MultiDep(10),
                MultiDep(10),
            ));
        }
    });
}

#[bench]
fn by_ref_dyn_from_multiple(b: &mut Bencher) {
    let value: &dyn MultiTrait = &Dep1(10);
    b.iter(move || {
        for _ in 0..ITERATION_COUNT {
            test::black_box((
                value, value, value, value, value, value, value, value, value, value,
            ));
        }
    });
}
