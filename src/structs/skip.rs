use crate::{Generator, GeneratorResult, ValueResult};

///
/// ## Example
///```
/// # use pipe_chan::{GenericGenerator, GeneratorExt, ValueResult};
/// # use pipe_chan::structs::Skip;
/// let input = [1,2,3,4];
/// let mut iter = input.iter();
/// let generator = GenericGenerator::new(|| iter.next());
/// let mut skipped_generator = Skip::new(generator, 2);
/// let mut output: Vec<i32> = Vec::new();
/// skipped_generator.for_each(|x| output.push(*x));
/// assert_eq!(output, [3,4]);
/// ```
pub struct Skip<Gen>
{
    generator: Gen,
    amount: usize,
}

impl<Gen> Skip<Gen>
{
    pub fn new(generator: Gen, amount: usize) -> Self {
        Self {
            generator,
            amount
        }
    }
}

impl<Gen> Generator for Skip<Gen>
where
    Gen: Generator
{
    type Output = Gen::Output;

    #[inline]
    fn run(&mut self, mut output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        if self.amount > 0 {
            let amount = &mut self.amount;
            let skip_run = self.generator.run(move |_| {
                *amount -= 1;
                (*amount != 0).into()
            });

            if skip_run == GeneratorResult::Complete {
                return GeneratorResult::Complete;
            }
            else if self.amount > 0 {
                return GeneratorResult::Stopped;
            }
        }

        self.generator.run(|value| output(value))
    }
}
