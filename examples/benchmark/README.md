# Benchmark
## Run
To run the benchmarks, use the following command on `nightly`:
```sh
cargo bench
```
## Output
```
test tests::baseline_by_ref         ... bench:      10,504 ns/iter (+/- 998)
test tests::baseline_by_ref_dyn     ... bench:      20,914 ns/iter (+/- 1,197)
test tests::baseline_by_value       ... bench:     134,250 ns/iter (+/- 9,113)
test tests::nject_by_module_ref     ... bench:      10,479 ns/iter (+/- 731)
test tests::nject_by_module_ref_dyn ... bench:      20,909 ns/iter (+/- 1,777)
test tests::nject_by_ref            ... bench:      10,473 ns/iter (+/- 366)
test tests::nject_by_ref_dyn        ... bench:      20,960 ns/iter (+/- 1,472)
test tests::nject_by_value          ... bench:     134,240 ns/iter (+/- 3,870)
```