use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pushgen::{SliceGenerator, GeneratorExt};
use itertools::Itertools;

fn run_iterator_for_each(data: &Vec<i32>) {
    let mut result = 0i32;
    data.iter()
        .dedup()
        .filter(|x| *x % 2 == 0)
        .for_each(|x| result = result.wrapping_add(*x));
    black_box(result);
}

fn run_iterator_next(data: &Vec<i32>) {
    let mut result = 0i32;
    for x in data.iter()
        .dedup()
        .filter(|x| *x % 2 == 0)
    {
        result = result.wrapping_add(*x);
    }
    black_box(result);
}

fn run_generator(data: &Vec<i32>) {
    let mut result = 0i32;
    SliceGenerator::new(data.as_slice())
        .dedup()
        .filter(|x| *x % 2 == 0)
        .for_each(|x| result = result.wrapping_add(*x));
    black_box(result);
}

pub fn make_data() -> Vec<i32> {
    let mut retval = Vec::new();
    retval.reserve(100_000);
    for x in 0..100_000/4 {
        retval.push(x);
        retval.push(x);
        retval.push(x);
        retval.push(x);
    }
    retval
}

pub fn benchmarks(c: &mut Criterion) {
    let data = make_data();
    c.bench_function("iterator_for_each_dedup_filter", |b| {
        b.iter(|| run_iterator_for_each(black_box(&data)))
    });
    c.bench_function("iterator_next_dedup_filter", |b| {
        b.iter(|| run_iterator_next(black_box(&data)))
    });
    c.bench_function("generator_dedup_filter", |b| {
        b.iter(|| run_generator(black_box(&data)))
    });
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
