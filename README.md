
# pushgen


[![Crates.io][crates-badge]][crates-url]
![https://docs.rs/pushgen](https://docs.rs/pushgen/badge.svg)

[crates-badge]: https://img.shields.io/crates/v/pushgen.svg
[crates-url]: https://crates.io/crates/pushgen

Push-style design pattern for processing of ranges and data-streams.

This is a Rust-based approach to the design pattern described by [transrangers](https://github.com/joaquintides/transrangers).
While the discussion linked targets C++, the same basic principle of pull-based iterators applies
to Rust as well (with some modifications since Rust doesn't have a concept of an `end` iterator
like C++ does).

## Example
```rust
# fn process(x: i32) {}
# let data = [1, 2, 3, 4, 5];

for item in data.iter().filter(|x| *x % 2 == 0).map(|x| x * 3) {
    process(item);
}
```

can be rewritten as
```rust
use pushgen::{SliceGenerator, GeneratorExt};
# fn process(_x: i32) {}
# let data = [1, 2, 3, 4, 5];
// Assume data is a slice
SliceGenerator::new(&data).filter(|x| *x % 2 == 0).map(|x| x * 3).for_each(process);
```

## Performance

I make no performance-claims, however there are some benchmarked cases where the push-based approach
wins over the iterator approach, but I have made no attempts to analyze this in any depth.