use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pushgen::{SliceGenerator, GeneratorExt};

fn run_iterator(data: &Vec<i32>) {
    let mut result = 0i32;
    data.iter()
        .zip(data.iter())
        .for_each(|x| result = result.wrapping_add(x.0 + x.1));
    black_box(result);
}

fn run_generator(data: &Vec<i32>) {
    let mut result = 0i32;
    SliceGenerator::new(data.as_slice())
        .zip(SliceGenerator::new(data.as_slice()))
        .for_each(|x| result = result.wrapping_add(x.0 + x.1));
    black_box(result);
}

pub fn make_data(amount: usize) -> Vec<i32> {
    let mut retval = Vec::new();
    retval.reserve(amount);
    for x in 0..amount {
        retval.push(x as i32);
    }
    retval
}

pub fn benchmarks(c: &mut Criterion) {
    let data = make_data(1000_000);
    c.bench_function("iterator_zip", |b| {
        b.iter(|| run_iterator(black_box(&data)))
    });
    c.bench_function("generator_zip", |b| {
        b.iter(|| run_generator(black_box(&data)))
    });
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
