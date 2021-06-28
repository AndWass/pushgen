use crate::{Generator, GeneratorResult, ValueResult};

/// Skip over a set amount of values. See [`.skip()`](crate::GeneratorExt::skip) for more details.
pub struct Skip<Gen>
{
    generator: Gen,
    amount: usize,
}

impl<Gen> Skip<Gen>
{
    pub(crate) fn new(generator: Gen, amount: usize) -> Self {
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
