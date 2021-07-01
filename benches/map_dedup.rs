use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pushgen::{SliceGenerator, GeneratorExt};
use itertools::Itertools;

fn run_iterator(data: &Vec<i32>) {
    let mut result = 0i32;
    data.iter()
        .map(|x| x * 3)
        .dedup()
        .for_each(|x| result = result.wrapping_add(x));
    black_box(result);
}

fn run_generator(data: &Vec<i32>) {
    let mut result = 0i32;
    SliceGenerator::new(data.as_slice())
        .map(|x| x * 3)
        .dedup()
        .for_each(|x| result = result.wrapping_add(x));
    black_box(result);
}

pub fn make_data(amount: usize) -> Vec<i32> {
    let mut retval = Vec::new();
    retval.reserve(amount);
    for x in 0..amount {
        if x % 100 == 0 {
            // Add some duplicates every 100 values
            for _ in 0..10 {
                retval.push(x as i32);
            }
        }
        retval.push(x as i32);
    }
    retval
}

pub fn benchmarks(c: &mut Criterion) {
    let data = make_data(1000_000);
    c.bench_function("iterator_map_dedup", |b| {
        b.iter(|| run_iterator(black_box(&data)))
    });
    c.bench_function("generator_map_dedup", |b| {
        b.iter(|| run_generator(black_box(&data)))
    });
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
