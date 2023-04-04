use criterion::{black_box, criterion_group, criterion_main, Criterion};
use itertools::Itertools;

fn run_iterator_try_for_each(data: &[i32]) {
    let mut result = 0i32;
    data.iter()
        .dedup()
        .filter(|x| *x % 2 == 0)
        .try_for_each(|x| {
            result = result.wrapping_add(*x);
            Some(())
        });
    black_box(result);
}

pub fn make_data() -> Vec<i32> {
    let mut retval = Vec::new();
    retval.reserve(100_000);
    for x in 0..100_000 / 4 {
        retval.push(x);
        retval.push(x);
        retval.push(x);
        retval.push(x);
    }
    retval
}

pub fn benchmarks(c: &mut Criterion) {
    let data = make_data();
    c.bench_function("iterator_try_for_each_dedup_filter", |b| {
        b.iter(|| run_iterator_try_for_each(black_box(&data)))
    });
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
