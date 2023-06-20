# Benchmark
## Run
To run the benchmarks, use the following command on `nightly`:
```sh
cargo bench
```
## Output
```
test tests::baseline_by_ref     ... bench:      10,784 ns/iter (+/- 1,581)
test tests::baseline_by_value   ... bench:     137,040 ns/iter (+/- 4,576)
test tests::nject_by_module_ref ... bench:      10,664 ns/iter (+/- 592)
test tests::nject_by_ref        ... bench:      10,531 ns/iter (+/- 279)
test tests::nject_by_value      ... bench:     136,988 ns/iter (+/- 3,023)
```