use crate::{Generator, GeneratorExt, GeneratorResult, ValueResult};
use std::num::NonZeroUsize;

pub struct StepBy<Src> {
    source: Src,
    // Always step size - 1
    advance_amount: usize,
    // Number of steps to advance stored from any previous runs
    amount_to_advance: usize,
}

impl<Src> StepBy<Src> {
    pub(crate) fn new(source: Src, step_size: usize) -> Self {
        if step_size == 0 {
            panic!("Step size must not be 0");
        }
        Self {
            source,
            advance_amount: step_size - 1,
            amount_to_advance: 0,
        }
    }
}

impl<Src: Generator> Generator for StepBy<Src> {
    type Output = Src::Output;

    #[inline]
    fn run(&mut self, mut output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        if self.advance_amount == 0 {
            self.source.run(output)
        } else {
            if self.amount_to_advance != 0 {
                // There are left-over advances needed from previous runs.
                // Safety: not zero
                match self
                    .source
                    .try_advance(unsafe { NonZeroUsize::new_unchecked(self.amount_to_advance) })
                {
                    (_, GeneratorResult::Complete) => return GeneratorResult::Complete,
                    (x, GeneratorResult::Stopped) => {
                        if x != self.amount_to_advance {
                            self.amount_to_advance -= x;
                            return GeneratorResult::Stopped;
                        }
                    }
                }
            }
            // Base case
            self.amount_to_advance = 0;
            loop {
                match self.source.next() {
                    Ok(x) => match output(x) {
                        ValueResult::Stop => {
                            // Make sure to advance in any coming runs.
                            self.amount_to_advance = self.advance_amount;
                            return GeneratorResult::Stopped;
                        }
                        ValueResult::MoreValues => {}
                    },
                    Err(err) => return err, // self.amount_to_advance already set to 0.
                }

                // Safety: self.advance_amount is never 0
                match self
                    .source
                    .try_advance(unsafe { NonZeroUsize::new_unchecked(self.advance_amount) })
                {
                    (_, GeneratorResult::Complete) => return GeneratorResult::Complete,
                    (x, _) => {
                        if x != self.advance_amount {
                            // Partial advance, store the number of left-over advances needed for future runs.
                            self.amount_to_advance = self.advance_amount - x;
                            return GeneratorResult::Stopped;
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::{MultiStoppingGen, StoppingGen};
    use crate::{GeneratorExt, GeneratorResult, IntoGenerator};

    #[test]
    fn basic_test() {
        let data = [0, 1, 2, 3, 4, 5];
        let mut output = Vec::new();

        let result = StepBy::new(data.into_gen(), 2).for_each(|x| {
            output.push(x);
        });

        assert_eq!(result, GeneratorResult::Complete);
        assert_eq!(output, [&0, &2, &4]);
    }

    #[test]
    fn spuriously_stopping() {
        let data = [0, 1, 2, 3, 4, 5];
        for stop_at in 0..6 {
            let mut output = Vec::new();
            let mut gen = StepBy::new(StoppingGen::new(stop_at, &data), 2);

            let result = gen.for_each(|x| output.push(x));
            assert_eq!(result, GeneratorResult::Stopped);
            let result = gen.for_each(|x| output.push(x));
            assert_eq!(result, GeneratorResult::Complete);
            assert_eq!(output, [&0, &2, &4]);
        }
    }

    #[test]
    fn spuriously_multi_stopping() {
        let stop_at = [0, 1, 3, 5];
        let data = [0, 1, 2, 3, 4, 5, 6];
        let mut output = Vec::new();
        let mut gen = StepBy::new(MultiStoppingGen::new(&stop_at, &data), 3);

        let result = gen.for_each(|x| output.push(x));
        assert_eq!(result, GeneratorResult::Stopped);
        let result = gen.for_each(|x| output.push(x));
        assert_eq!(result, GeneratorResult::Stopped);
        let result = gen.for_each(|x| output.push(x));
        assert_eq!(result, GeneratorResult::Stopped);
        let result = gen.for_each(|x| output.push(x));
        assert_eq!(result, GeneratorResult::Stopped);
        let result = gen.for_each(|x| output.push(x));
        assert_eq!(result, GeneratorResult::Complete);
        assert_eq!(output, [&0, &3, &6]);
    }

    #[test]
    fn step_one() {
        let data = [0, 1, 2, 3, 4, 5];
        let mut output = Vec::new();

        let result = StepBy::new(data.into_gen(), 1).for_each(|x| {
            output.push(x);
        });

        assert_eq!(result, GeneratorResult::Complete);
        assert_eq!(output, [&0, &1, &2, &3, &4, &5]);
    }

    #[test]
    fn step_next() {
        let data = [0, 1, 2, 3, 4, 5];

        let mut gen = data.into_gen().step_by(2);
        assert_eq!(gen.next(), Ok(&0));
        assert_eq!(gen.next(), Ok(&2));
        assert_eq!(gen.next(), Ok(&4));
    }

    #[test]
    #[should_panic]
    fn zero_step_size() {
        let data = [0, 1, 2];
        let _gen = data.into_gen().step_by(0);
    }
}
