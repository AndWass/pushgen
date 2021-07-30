use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn run_iterator_for_each(data: &Vec<i32>, step_size: usize) {
    let mut result = 0i32;
    data.iter().step_by(step_size).for_each(|x| {
        result = result.wrapping_add(*x);
    });
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
    c.bench_function("iterator_for_each_step_by_2", |b| {
        b.iter(|| run_iterator_for_each(black_box(&data), 2))
    });

    c.bench_function("iterator_for_each_step_by_10", |b| {
        b.iter(|| run_iterator_for_each(black_box(&data), 10))
    });

    c.bench_function("iterator_for_each_step_by_100", |b| {
        b.iter(|| run_iterator_for_each(black_box(&data), 100))
    });

    c.bench_function("iterator_for_each_step_by_1000", |b| {
        b.iter(|| run_iterator_for_each(black_box(&data), 1000))
    });
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
