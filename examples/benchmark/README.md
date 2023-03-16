# Benchmark
## Run
To run the benchmarks, use the following command on `nightly`:
```sh
cargo bench
```
## Output
```
test tests::baseline_by_ref   ... bench:      10,614 ns/iter (+/- 1,318)
test tests::baseline_by_value ... bench:     135,785 ns/iter (+/- 42,243)
test tests::nject_by_ref      ... bench:      10,521 ns/iter (+/- 436)
test tests::nject_by_value    ... bench:     135,441 ns/iter (+/- 24,549)
```