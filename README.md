# Rust Benchmarks

Results on my PC (x86 i5-12400F CPU).

Read/write usize values
```
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

Orange pi 5 (ARM64 RK3588)
```
test test_array_read                      ... bench:   1,576,372.00 ns/iter (+/- 63,110.42)
test test_array_read_atomic               ... bench:   1,569,423.18 ns/iter (+/- 56,681.45)
test test_array_read_atomic_no_blackbox   ... bench:   1,156,904.40 ns/iter (+/- 59,151.90)
test test_array_read_no_blackbox          ... bench:     982,381.53 ns/iter (+/- 21,811.18)
test test_array_read_refcell              ... bench:   2,937,680.12 ns/iter (+/- 98,168.15)
test test_array_read_refcell_no_blackbox  ... bench:   1,452,867.95 ns/iter (+/- 143,110.26)
test test_array_write                     ... bench:   1,071,185.10 ns/iter (+/- 21,445.94)
test test_array_write_atomic              ... bench:   1,364,888.30 ns/iter (+/- 1,225.11)
test test_array_write_atomic_no_blackbox  ... bench:     909,313.10 ns/iter (+/- 1,061.66)
test test_array_write_no_blackbox         ... bench:     303,077.30 ns/iter (+/- 36,414.12)
test test_array_write_refcell             ... bench:   2,181,562.28 ns/iter (+/- 454,601.05)
test test_array_write_refcell_no_blackbox ... bench:   2,057,044.70 ns/iter (+/- 278,934.12)
```