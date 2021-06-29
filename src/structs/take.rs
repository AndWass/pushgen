use crate::{Generator, GeneratorResult, ValueResult};

/// Take `n` values from a generator. See [`.take()`](crate::GeneratorExt::take) for details.
pub struct Take<Src> {
    source: Src,
    amount_left: usize,
}

impl<Src: Generator> Take<Src> {
    pub fn new(source: Src, amount: usize) -> Self {
        Self {
            source,
            amount_left: amount,
        }
    }
}

impl<Src: Generator> Generator for Take<Src> {
    type Output = Src::Output;

    #[inline]
    fn run(&mut self, mut output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        if self.amount_left > 0 {
            let amount_left = &mut self.amount_left;
            let result = self.source.run(|x| {
                *amount_left -= 1;
                let res = output(x);
                if *amount_left == 0 {
                    ValueResult::Stop
                } else {
                    res
                }
            });
            if result == GeneratorResult::Complete {
                self.amount_left = 0;
                return GeneratorResult::Complete;
            }
            if self.amount_left == 0 {
                return GeneratorResult::Complete;
            }
            return result;
        }
        GeneratorResult::Complete
    }
}

#[cfg(test)]
mod tests {
    use crate::structs::Take;
    use crate::{Generator, GeneratorResult, ValueResult, SliceGenerator};

    #[test]
    fn take() {
        let data = [1, 2, 3, 4, 5];
        let mut output: Vec<i32> = Vec::new();

        let result = Take::new(SliceGenerator::new(&data), 2).run(|x| {
            output.push(*x);
            ValueResult::MoreValues
        });
        assert_eq!(result, GeneratorResult::Complete);
        assert_eq!(output, [1, 2]);
    }

    #[test]
    fn take_restart() {
        let data = [1, 2, 3, 4, 5];
        let mut output: Vec<i32> = Vec::new();

        let mut generator = Take::new(SliceGenerator::new(&data), 4);

        let result = generator.run(|x| {
            output.push(*x);
            (output.len() < 2).into()
        });

        assert_eq!(result, GeneratorResult::Stopped);
        assert_eq!(output, [1, 2]);

        let result = generator.run(|x| {
            output.push(*x);
            ValueResult::MoreValues
        });
        assert_eq!(result, GeneratorResult::Complete);
        assert_eq!(output, [1,2,3,4]);
    }
}
