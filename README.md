# Music Player Algorithm
I did this coding test for a job application last week but the problem stuck with me, so here's my original (`nested_for`) and better (`pivot`) solution.

## Rationale
There's a music player that plays songs in pairs, but the pairs had to have a total time a multiple of a minute. An array of songs with the time in seconds is provided. And the algorithm should output the number of combinaions possible.

# Implementation Notes

## Original Implementation (`nested_for`)
I wrote the original in a few minutes at a Starbucks while on a date and felt the simpler the better and didn't think too much about it.

It was O(n^2) time with O(1) memory to run, and O(1) memory to print all the combos if need be. It was ridiculously simple and intuitive.

## Better Implementation (`pivot`)
Today I was in the shower and my mind drifted back to the problem. I thought about implementing it in a pivot style, where all numbers were stripped modulo 60 and I can iterate through and obtain the combination simply by multiplying 2 numbers that add up to 60. The special cases were 0 and 30, which when added together modulo 60 would equal 0, so they were excluded from the final iterator.

It is now O(n) with O(1) memory to run, and O(n) memory to print all the combos if need be (restructure the `sorted` counts array to `[[song_length]; 60]`). More difficult to explain but that's that. Often the first thing that comes to mind isn't the best and if it gets submitted unchecked it would end in the disaster that says "your application was strong ~~but you failed our coding test~~". (They didn't actually say that,)

# Results

## `cargo bench`
Using rust `nightly-x86_64-pc-windows-msvc` on a Windows 10 Pro computer with an AMD Ryzen 5600X 4.6GHz CPU.

```rust
$ cargo bench
    Finished bench [optimized] target(s) in 0.01s
     Running unittests src\main.rs (target\release\deps\mp_alg-50a8af37986043bd.exe)

running 11 tests
test tests::nested_for ... ignored
test tests::pivot_base_case_0 ... ignored
test tests::pivot_base_case_30 ... ignored
test tests::pivot_blackbox ... ignored
test tests::pivot_idempotency ... ignored
test bench::nested_for_100   ... bench:       2,466 ns/iter (+/- 46)
test bench::nested_for_1000  ... bench:     218,732 ns/iter (+/- 5,645)
test bench::nested_for_10000 ... bench:  24,369,130 ns/iter (+/- 312,058)
test bench::pivot_100        ... bench:          66 ns/iter (+/- 1)
test bench::pivot_1000       ... bench:         580 ns/iter (+/- 17)
test bench::pivot_10000      ... bench:       5,971 ns/iter (+/- 192)

test result: ok. 0 passed; 0 failed; 5 ignored; 6 measured; 0 filtered out; finished in 14.80s

```

Observe the exponential growth of `nested_for` and linear growth of `pivot`! If only I came up with it last week... xD
