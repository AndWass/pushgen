use pipe_chan::generator::SliceGenerator;
use pipe_chan::{GeneratorExt, GeneratorResult};

fn make_data() -> Vec<i32> {
    let mut data = Vec::new();
    data.extend(
        std::iter::repeat(0)
            .enumerate()
            .map(|(x, _)| x as i32)
            .take(1000),
    );
    data
}

fn main() {
    let data = make_data();

    let iter_sum: i32 = data.iter().filter(|x| *x % 2 == 0).map(|x| x * 3).sum();

    let mut generator_sum = 0i32;
    let generator_result = SliceGenerator::new(data.as_slice())
        .filter(|x| *x % 2 == 0)
        .map(|x| x * 3)
        .for_each(|x| generator_sum += x);

    assert_eq!(generator_result, GeneratorResult::Complete);
    assert_eq!(iter_sum, generator_sum);

    println!("Iterator sum = {}", iter_sum);
    println!("Generator sum = {}", generator_sum);
}
