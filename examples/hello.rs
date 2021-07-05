use pushgen::{GeneratorExt, SliceGenerator};

fn main() {
    let data = [1, 2, 3, 4];
    SliceGenerator::new(&data)
        .map(|x| format!("Hello {}", x))
        .for_each(|x| println!("{}", x));
}
