use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn run_iterator_for_each(data: &[i32]) {
    let mut result = 0i32;
    data.iter()
        .filter(|x| *x % 2 == 0)
        .map(|x| x * 3)
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
    let data = make_data(1_000_000);
    c.bench_function("iterator_for_each_filter_map", |b| {
        b.iter(|| run_iterator_for_each(black_box(&data)))
    });
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
