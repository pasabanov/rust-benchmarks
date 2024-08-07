# Rust Benchmarks

Results on my PC (i5-12400F CPU, Linux).

Read/write usize values
```cookie
test test_array_read                      ... bench:     548,736.57 ns/iter (+/- 5,462.46)
test test_array_read_atomic               ... bench:     547,524.40 ns/iter (+/- 29,801.87)
test test_array_read_atomic_no_blackbox   ... bench:     261,458.35 ns/iter (+/- 19,232.90)
test test_array_read_no_blackbox          ... bench:     258,185.13 ns/iter (+/- 7,763.45)
test test_array_read_refcell              ... bench:   1,887,381.75 ns/iter (+/- 25,029.11)
test test_array_read_refcell_no_blackbox  ... bench:     615,552.38 ns/iter (+/- 26,502.84)
test test_array_write                     ... bench:     737,569.30 ns/iter (+/- 21,486.75)
test test_array_write_atomic              ... bench:     737,195.50 ns/iter (+/- 28,456.04)
test test_array_write_atomic_no_blackbox  ... bench:     230,725.72 ns/iter (+/- 8,196.04)
test test_array_write_no_blackbox         ... bench:     213,745.58 ns/iter (+/- 6,939.20)
test test_array_write_refcell             ... bench:   1,109,131.04 ns/iter (+/- 76,639.49)
test test_array_write_refcell_no_blackbox ... bench:     727,204.07 ns/iter (+/- 21,522.50
```