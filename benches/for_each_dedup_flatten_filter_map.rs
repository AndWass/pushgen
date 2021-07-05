use criterion::{black_box, criterion_group, criterion_main, Criterion};
use itertools::Itertools;

fn run_iterator_for_each(data: &Vec<Vec<i32>>) {
    let mut result = 0i32;
    data.iter()
        .map(|x| x.iter().dedup())
        .flatten()
        .filter(|x| *x % 2 == 0)
        .map(|x| x * 3)
        .for_each(|x| result = result.wrapping_add(x));
    black_box(result);
}

pub fn make_data() -> Vec<Vec<i32>> {
    let mut data = Vec::new();
    data.reserve(100_000);
    for x in 0..100_000 / 4 {
        data.push(x);
        data.push(x);
        data.push(x);
        data.push(x);
    }

    let mut retval = Vec::new();
    for _x in 0..10 {
        retval.push(data.clone());
    }
    retval
}

pub fn benchmarks(c: &mut Criterion) {
    let data = make_data();
    c.bench_function("iterator_for_each_dedup_flatten_filter_map", |b| {
        b.iter(|| run_iterator_for_each(black_box(&data)))
    });
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
