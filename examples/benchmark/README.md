# Benchmark
## Run
To run the benchmarks, use the following command:
```sh
cargo +nightly bench
```
## Output
```
test tests::baseline_by_ref         ... bench:      10,540 ns/iter (+/- 364)
test tests::baseline_by_value       ... bench:     135,060 ns/iter (+/- 9,365)
test tests::nject_by_ref            ... bench:      10,594 ns/iter (+/- 786)
test tests::nject_by_value          ... bench:     135,058 ns/iter (+/- 6,818)
test tests::nject_extended_by_ref   ... bench:      10,536 ns/iter (+/- 357)
test tests::nject_extended_by_value ... bench:     134,745 ns/iter (+/- 5,582)
```