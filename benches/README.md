## Benchmarks

These are informal benchmarks using Rust 1.53.0 Stable on OpenSuse
Tumbleweed on my personal computer.

I have benchmarked `pushgen` against Rust iterators together with the popular 
`Itertools` crate,
[transrangers](https://github.com/joaquintides/transrangers) using
g++ 11.1.1 and clang 12.0.0.

Rust benchmarks were run using `taskset 0x01 cargo criterion --bench <name>`.
The C++ benchmarks were built using `g++ -O3 -DNDEBUG -finline-limit=10000` and
`clang++ -O3 -DNDEBUG`. They were both run using `taskset 0x01 <binary>`.

The tests below map against the different test benchmarks done by `transrangers`
according to this table:

Test   | Implementation file         |
-------|-----------------------------|
Test 1 | filter_map.rs               |
Test 2 | chain_take_filter_map.rs    |
Test 3 | dedup_filter.rs             |
Test 4 | flatten_dedup_filter_map.rs |
Test 5 | dedup_flatten_filter_map.rs |
Test 6 | transrangers_test6.rs       |


All values are the benchmarked elapsed time in micro-seconds.

Test | g++ Transrangers | g++ Ranges-v3 | clang Transrangers | clang Ranges-v3 | Iterator for_each | Iterator try_for_each | Iterator next | pushgen |
-----|------------------|---------------|--------------------|-----------------|-------------------|-----------------------|---------------|---------|
1    | 211              | 512           | 176                | 511             | 171               | 829                   | 422           | 174     |
2    | 1091             | 4075          | 1056               | 7231            | 2743              | 2834                  | 2911          | 923     |
3    | 24               | 73            | 28                 | 88              | 93                | 38                    | 65            | 24      |
4    | 752              | 603           | 249                | 896             | 465               | 1022                  | 1901          | 304     |
5    | 286              | 997           | 270                | 954             | 324               | 730                   | 675           | 306     |
6    | 931              | 1016          | 345                | 1199            | 353               | 1089                  | 751           | 356     |

## Comments

Both the standard library and `itertools` are leaving tons of performance on the table. `try_for_each` should
in theory be the same as pushgen but both the standard library and `itertools` are missing `#[inline]` directives
here and there. In the case of `Itertools::dedup()` there is no bespoke
`try_fold` implementation, only a (non-inlined) `fold` (for instance).