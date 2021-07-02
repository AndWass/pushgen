## Benchmarks

These are informal benchmarks using Rust 1.53.0 Stable on OpenSuse
Tumbleweed on my personal computer.

I have benchmarked `pushgen` against Rust iterators and
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

Test    | g++ Transrangers | g++ Ranges-v3 | clang Transrangers | clang Ranges-v3 | Rust Iterator::for_each | Rust iterator::next | pushgen |
--------|------------------|---------------|--------------------|-----------------|-------------------------|---------------------|---------|
Test 1  | 211              | 512           | 176                | 511             | 184                     | 431                 | 180     |
Test 2  | 1091             | 4075          | 1056               | 7231            | 1085                    | 2895                | 835     |
Test 3  | 24               | 73            | 28                 | 88              |  98                     | 63                  | 24      |
Test 4  | 752              | 603           | 249                | 896             | 470                     | 1950                | 312     |
Test 5  | 286              | 997           | 270                | 954             | 318                     | 662                 | 303     |
Test 6  | 931              | 1016          | 345                | 1199            | 369                     | 784                 | 368     |

