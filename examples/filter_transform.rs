use pipe_chan::StageExt;
use pipe_chan::InputStage;

fn by_hand(data: &Vec<i32>) -> i32 {
    let mut retval = 0i32;
    for x in data {
        if x % 2 == 0 {
            retval = retval.wrapping_add(x*3);
        }
    }
    retval
}

fn by_iterator(data: &Vec<i32>) -> i32 {
    let mut result = 0i32;
    data.iter().filter(|x| **x % 2 == 0).map(|x| x * 3).for_each(|x| result = result.wrapping_add(x));
    result
}

fn pipeline(data: &Vec<i32>) -> i32 {
    let mut result = 0i32;
    let mut pipe = pipe_chan::begin::<i32>()
        .filter(|x| (*x % 2) == 0)
        .transform(|x| x * 3)
        .end(|x| {
            result = result.wrapping_add(x);
            true
        });
    for elem in data {
        pipe.process(*elem);
    }

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

    let mut by_hand_duration = std::time::Duration::new(0, 0);
    let mut iterator_duration = std::time::Duration::new(0, 0);
    let mut pipeline_duration = std::time::Duration::new(0, 0);

    let mut sum = 0i32;

    for _ in 0..cycles {
        let begin = std::time::Instant::now();
        sum = sum.wrapping_add(pipeline(&data));
        let end = std::time::Instant::now();
        pipeline_duration += end - begin;
    }

    for _ in 0..cycles {
        let begin = std::time::Instant::now();
        sum = sum.wrapping_add(by_hand(&data));
        let end = std::time::Instant::now();
        by_hand_duration += end - begin;
    }

    for _ in 0..cycles {
        let begin = std::time::Instant::now();
        sum = sum.wrapping_add(by_iterator(&data));
        let end = std::time::Instant::now();

        iterator_duration += end - begin;
    }

    println!("Count = {}, Cycles = {}", count, cycles);
    println!("By hand time: {:?}", by_hand_duration/cycles);
    println!("Iterator time: {:?}", iterator_duration/cycles);
    println!("Pipeline time: {:?}", pipeline_duration/cycles);
    println!("Result: {}", sum);
}
