# Benchmark
## Run
To run the benchmarks, use the following command on `nightly`:
```sh
cargo bench
```
baseline = The performance we want to achieve.
## Output
```
// BASELINE
test baseline::by_ref                 ... bench:      10,566 ns/iter (+/- 366)
test baseline::by_ref_dyn             ... bench:      20,998 ns/iter (+/- 3,899)
test baseline::by_ref_from_scope_root ... bench:      10,522 ns/iter (+/- 722)
test baseline::by_value               ... bench:     136,151 ns/iter (+/- 6,654)
test baseline::by_value_from_scope    ... bench:     150,551 ns/iter (+/- 6,731)
// NJECT
test module::by_ref_dyn_from_module   ... bench:      20,937 ns/iter (+/- 2,002)
test module::by_ref_from_module       ... bench:      10,470 ns/iter (+/- 1,244)
test module::by_ref_impl_from_module  ... bench:      10,776 ns/iter (+/- 1,539)
test provide::by_ref                  ... bench:      10,501 ns/iter (+/- 597)
test provide::by_ref_dyn              ... bench:      20,902 ns/iter (+/- 3,129)
test provide::by_ref_impl             ... bench:      10,470 ns/iter (+/- 1,963)
test provide::by_value                ... bench:     136,128 ns/iter (+/- 8,489)
test scope::by_ref_from_root          ... bench:      10,463 ns/iter (+/- 2,337)
test scope::by_value_from_scope       ... bench:     143,150 ns/iter (+/- 7,108)
```