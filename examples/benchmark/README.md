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
test baseline::by_ref                    ... bench:      10,783.83 ns/iter (+/- 205.29)
test baseline::by_ref_dyn                ... bench:      22,631.05 ns/iter (+/- 3,664.00)
test baseline::by_ref_dyn_from_multiple  ... bench:      24,612.75 ns/iter (+/- 3,476.90)
test baseline::by_ref_from_scope_root    ... bench:      10,743.33 ns/iter (+/- 1,268.99)
test baseline::by_value                  ... bench:     158,883.33 ns/iter (+/- 22,434.83)
test baseline::by_value_from_multiple    ... bench:       4,895.17 ns/iter (+/- 1,083.56)
test baseline::by_value_from_scope       ... bench:     141,726.67 ns/iter (+/- 3,514.92)
test baseline::iter_by_dyn_ref           ... bench:       2,142.40 ns/iter (+/- 56.83)
test baseline::iter_by_value             ... bench:       2,456.25 ns/iter (+/- 407.69)
// NJECT
test module::by_ref_from_module          ... bench:      10,747.12 ns/iter (+/- 623.08)
test module::by_ref_dyn_from_module      ... bench:      21,733.28 ns/iter (+/- 4,648.66)
test module::iter_by_dyn_ref_from_module ... bench:       2,142.08 ns/iter (+/- 139.79)
test module::iter_by_value_from_module   ... bench:       2,140.57 ns/iter (+/- 74.31)
test provide::by_ref                     ... bench:      10,721.28 ns/iter (+/- 1,173.49)
test provide::by_ref_dyn                 ... bench:      21,396.50 ns/iter (+/- 600.25)
test provide::by_ref_dyn_from_multiple   ... bench:      23,263.33 ns/iter (+/- 5,029.95)
test provide::by_value                   ... bench:     139,292.00 ns/iter (+/- 3,571.00)
test provide::by_value_from_multiple     ... bench:       4,891.21 ns/iter (+/- 900.87)
test scope::by_ref_dyn_from_multiple     ... bench:      21,467.38 ns/iter (+/- 258.00)
test scope::by_ref_from_root             ... bench:      10,681.46 ns/iter (+/- 342.32)
test scope::by_value_from_multiple       ... bench:       3,674.34 ns/iter (+/- 653.49)
test scope::by_value_from_scope          ... bench:     140,853.33 ns/iter (+/- 4,829.33)
```