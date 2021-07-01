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

Test    | g++ Transrangers | g++ Ranges-v3 | clang Transrangers | clang Ranges-v3 | Rust iterators | pushgen |
--------|------------------|---------------|--------------------|-----------------|----------------|---------|
Test 1  | 203              | 485           | 174                | 524             | 174            | 174     |
Test 2  | 1080             | 4011          | 1039               | 7075            | 1267           | 813     |
Test 3  | 24               | 71            | 22                 | 77              |  94            | 94      |
Test 4  | 700              | 602           | 253                | 887             | 520            | 271     |
Test 5  | 283              | 1011          | 266                | 938             | 237            | 174     |
Test 6  | 899              | 1002          | 347                | 1172            | 348            | 348     |

