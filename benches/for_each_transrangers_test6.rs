use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn run_iterator_for_each(data: &[i32]) {
    let mut result = 0i32;
    data.iter()
        .zip(data.iter().map(|x| x * 3))
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
    let data = make_data(1_000_000);
    c.bench_function("iterator_for_each_transrangers_test6", |b| {
        b.iter(|| run_iterator_for_each(black_box(&data)))
    });
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
