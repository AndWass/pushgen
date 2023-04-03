use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pushgen::{GeneratorExt, SliceGenerator};

fn function_under_bench(data: &Vec<i32>) {
    let mut result = 0i32;

    for x in SliceGenerator::new(data.as_slice()).iter() {
        result = result.wrapping_add(*x);
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
    let data = make_data(1_000_000);
    c.bench_function("pushgen_iter_basic_loop", |b| {
        b.iter(|| function_under_bench(black_box(&data)))
    });
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
