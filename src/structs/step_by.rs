use crate::{Generator, GeneratorResult, ValueResult};

/// A generator for stepping values by a custom amount. See [`.step_by()`](crate::GeneratorExt::step_by) for details.
pub struct StepBy<Src> {
    source: Src,
    step: usize,
    index: usize,
    first_output: bool,
}

impl<Src> StepBy<Src>
where
    Src: Generator,
{
    #[inline]
    pub(crate) fn new(source: Src, step: usize) -> Self {
        if step == 0 {
            panic!("Step size in StepBy must not be 0")
        }

        Self {
            source,
            step,
            index: 0,
            first_output: true,
        }
    }

    fn take_one(&mut self) -> Result<Src::Output, GeneratorResult> {
        let mut retval = Err(GeneratorResult::Stopped);
        match self.source.run(|x| {
            retval = Ok(x);
            ValueResult::Stop
        }) {
            GeneratorResult::Complete => Err(GeneratorResult::Complete),
            _ => retval,
        }
    }
}

impl<Src> Generator for StepBy<Src>
where
    Src: Generator,
{
    type Output = Src::Output;

    #[inline]
    fn run(&mut self, mut output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        if self.first_output {
            let out_result = match self.take_one() {
                Ok(x) => {
                    self.first_output = false;
                    output(x)
                },
                Err(x) => return x,
            };

            if out_result == ValueResult::Stop {
                return GeneratorResult::Stopped;
            }
        }

        let mut index = self.index;
        let step = self.step;
        let ret = self.source.run(|x| {
            index += 1;
            if index == step {
                index = 0;
                output(x)
            }
            else {
                ValueResult::MoreValues
            }
        });

        self.index = index;

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
    fn step_one() {
        let data = [0, 1, 2, 3, 4, 5];
        let mut output = Vec::new();

        let result = StepBy::new(data.into_gen(), 1).for_each(|x| {
            output.push(x);
        });

        assert_eq!(result, GeneratorResult::Complete);
        assert_eq!(output, [&0, &1, &2, &3, &4, &5]);
    }
}
