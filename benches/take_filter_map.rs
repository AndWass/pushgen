use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pushgen::{SliceGenerator, GeneratorExt};

fn run_iterator(data: &Vec<i32>) {
    let mut result = 0i32;
    data.iter()
        .take(data.len()/2)
        .filter(|x| *x % 2 == 0)
        .map(|x| x * 3)
        .for_each(|x| result = result.wrapping_add(x));
    black_box(result);
}

fn run_generator(data: &Vec<i32>) {
    let mut result = 0i32;
    SliceGenerator::new(data.as_slice())
        .take(data.len() / 2)
        .filter(|x| *x % 2 == 0)
        .map(|x| x * 3)
        .for_each(|x| result = result.wrapping_add(x));
    black_box(result);
}

fn run_handwritten(data: &Vec<i32>) {
    let mut result = 0i32;
    for x in &data[0..data.len()/2] {
        if x % 2 == 0 {
            result = result.wrapping_add(x*3);
        }
    }
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
    c.bench_function("iterator_take_filter_map", |b| b.iter(|| run_iterator(black_box(&data))));
    c.bench_function("generator_take_filter_map", |b| b.iter(|| run_generator(black_box(&data))));
    c.bench_function("handwritten_take_filter_map", |b| b.iter(|| run_handwritten(black_box(&data))));
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
