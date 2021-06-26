use pipe_chan::{Generator, ValueResult, GeneratorResult, Combine};
use pipe_chan::generator::structs::{Skip};
use pipe_chan::value::structs::{Filter, Transform};

fn by_hand(data: &Vec<i32>) -> i32 {
    let mut retval = 100i32;
    for x in data {
        if x % 2 == 0 {
            retval = retval.wrapping_add(x * 3);
        }
    }
    retval
}

fn by_iterator(data: &Vec<i32>) -> i32 {
    let mut result = 0i32;
    data.iter()
        .skip(100)
        .filter(|x| **x % 2 == 0)
        .map(|x| x * 3)
        .for_each(|x| result = result.wrapping_add(x));
    result
}

struct VectorGenerator<'a> {
    index: usize,
    data: &'a Vec<i32>,
}

impl<'a> Generator for VectorGenerator<'a> {
    type Output = i32;

    fn run(&mut self, mut output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        while self.index < self.data.len() {
            if output(*unsafe { self.data.get_unchecked(self.index) }) == ValueResult::Stop {
                self.index += 1;
                return GeneratorResult::Stopped;
            }
            self.index += 1;
        }
        GeneratorResult::Complete
    }
}

fn stream(data: &Vec<i32>) -> i32 {
    let mut result = 0i32;
    let generator = Skip::new(VectorGenerator {
        index: 0,
        data,
    }, 100);
    let filter_transform = Combine::new(
        Filter::new(|x| x % 2 == 0),
        Transform::new(|x| x * 3));
    let mut generator = Combine::new(generator, filter_transform);
    generator.run(|x| {
        result = result.wrapping_add(x);
        ValueResult::MoreValues
    });
    result
}

fn main() {
    let arg = std::env::args().skip(1).collect::<Vec<String>>();

    let mut count = 1000_000;
    let mut cycles = 100;
    if let Some(arg) = arg.get(0) {
        count = arg.parse().unwrap();
    }
    if let Some(arg) = arg.get(1) {
        cycles = arg.parse().unwrap();
    }
    let mut data = Vec::<i32>::new();
    for x in 0..count {
        data.push(x);
    }

    let mut sum = 0i32;

    let mut stream_duration = std::time::Duration::new(0, 0);
    for _ in 0..cycles {
        let begin = std::time::Instant::now();
        sum = sum.wrapping_add(stream(&data));
        let end = std::time::Instant::now();
        stream_duration += end - begin;
    }

    let mut by_hand_duration = std::time::Duration::new(0, 0);
    for _ in 0..cycles {
        let begin = std::time::Instant::now();
        sum = sum.wrapping_add(by_hand(&data));
        let end = std::time::Instant::now();
        by_hand_duration += end - begin;
    }

    let mut iterator_duration = std::time::Duration::new(0, 0);
    for _ in 0..cycles {
        let begin = std::time::Instant::now();
        sum = sum.wrapping_add(by_iterator(&data));
        let end = std::time::Instant::now();

        iterator_duration += end - begin;
    }

    println!("Count = {}, Cycles = {}", count, cycles);
    println!("By hand time: {:?}", by_hand_duration / cycles);
    println!("Iterator time: {:?}", iterator_duration / cycles);
    println!("Stream time: {:?}", stream_duration / cycles);
    println!("Result: {}", sum);
}
