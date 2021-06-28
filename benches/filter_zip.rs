use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pushgen::{SliceGenerator, GeneratorExt};

fn run_handwritten(data: &Vec<i32>) {
    let mut result = 0i32;
    for x in data {
        let y = x + 3*x;
        if y % 3 == 0 {
            result = result.wrapping_add(y);
        }
    }
    black_box(result);
}

fn run_iterator(data: &Vec<i32>) {
    let mut result = 0i32;
    data.iter()
        .zip(data.iter().map(|x| x * 3))
        .map(|(a, b)| a + b)
        .filter(|x| x % 3 == 0)
        .for_each(|x| result = result.wrapping_add(x));
    black_box(result);
}

fn run_generator(data: &Vec<i32>) {
    let mut result = 0i32;
    SliceGenerator::new(data.as_slice())
        .zip(SliceGenerator::new(data.as_slice()).map(|x| x*3))
        .map(|(a, b)| a + b)
        .filter(|x| x % 3 == 0)
        .for_each(|x| result = result.wrapping_add(x));
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
    c.bench_function("iterator_filter_zip", |b| {
        b.iter(|| run_iterator(black_box(&data)))
    });
    c.bench_function("generator_filter_zip", |b| {
        b.iter(|| run_generator(black_box(&data)))
    });
    c.bench_function("handwritten_filter_zip", |b| {
        b.iter(|| run_handwritten(black_box(&data)))
    });
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
