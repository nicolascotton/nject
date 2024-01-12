# Benchmark
## Run
To run the benchmarks, use the following command on `nightly`:
```sh
cargo bench
```
## Output
```
test baseline::by_ref               	... bench:      11,314 ns/iter (+/- 2,010)
test baseline::by_ref_dyn           	... bench:      22,470 ns/iter (+/- 1,010)
test baseline::by_ref_from_scope_root   ... bench:      12,005 ns/iter (+/- 1,864)
test baseline::by_value             	... bench:     143,465 ns/iter (+/- 8,597)
test baseline::by_value_from_scope  	... bench:     155,505 ns/iter (+/- 11,400)
test module::by_ref_dyn_from_module 	... bench:      21,361 ns/iter (+/- 839)
test module::by_ref_from_module     	... bench:      10,523 ns/iter (+/- 397)
test provide::by_ref                	... bench:      10,565 ns/iter (+/- 586)
test provide::by_ref_dyn            	... bench:      21,150 ns/iter (+/- 1,308)
test provide::by_ref_impl           	... bench:      10,635 ns/iter (+/- 1,704)
test provide::by_value              	... bench:     134,386 ns/iter (+/- 11,089)
test scope::by_ref_from_root        	... bench:      10,504 ns/iter (+/- 908)
test scope::by_value_from_scope     	... bench:     141,212 ns/iter (+/- 5,487)
```