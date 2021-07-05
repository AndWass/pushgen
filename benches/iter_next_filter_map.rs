use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn run_iterator_next(data: &Vec<i32>) {
    let mut result = 0i32;
    for x in data.iter().filter(|x| *x % 2 == 0).map(|x| x * 3) {
        result = result.wrapping_add(x);
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
    c.bench_function("iterator_next_filter_map", |b| {
        b.iter(|| run_iterator_next(black_box(&data)))
    });
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
