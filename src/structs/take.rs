use crate::{Generator, GeneratorResult, ValueResult};

/// Take `n` values from a generator. See [`.take()`](crate::GeneratorExt::take) for details.
#[derive(Clone)]
pub struct Take<Src> {
    source: Src,
    amount_left: usize,
}

impl<Src: Generator> Take<Src> {
    #[inline]
    pub(crate) fn new(source: Src, amount: usize) -> Self {
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

/// A generator that only forwards values while the predicate returns `true`. See [`.take_while()`](crate::GeneratorExt::take_while) for details.
#[derive(Clone)]
pub struct TakeWhile<Src, P> {
    source: Src,
    predicate: P,
    is_complete: bool,
}

impl<Src, P> TakeWhile<Src, P>
where
    Src: Generator,
    P: FnMut(&Src::Output) -> bool,
{
    #[inline]
    pub(crate) fn new(source: Src, predicate: P) -> Self {
        Self {
            source,
            predicate,
            is_complete: false,
        }
    }
}

impl<Src, P> Generator for TakeWhile<Src, P>
where
    Src: Generator,
    P: FnMut(&Src::Output) -> bool,
{
    type Output = Src::Output;

    #[inline]
    fn run(&mut self, mut output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        let is_complete = &mut self.is_complete;
        if *is_complete {
            return GeneratorResult::Complete;
        }

        let predicate = &mut self.predicate;
        let result = self.source.run(|x| {
            if predicate(&x) {
                output(x)
            } else {
                *is_complete = true;
                ValueResult::Stop
            }
        });

        if *is_complete {
            GeneratorResult::Complete
        } else {
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::structs::take::TakeWhile;
    use crate::structs::Take;
    use crate::test::StoppingGen;
    use crate::{Generator, GeneratorExt, GeneratorResult, SliceGenerator, ValueResult};

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
    fn spuriously_stopping_take() {
        let data = [1, 2, 3, 4, 5];
        for x in 0..3 {
            let mut output: Vec<i32> = Vec::new();
            let mut gen = StoppingGen::new(x, &data).take(3);

            let result = gen.for_each(|x| output.push(*x));
            assert_eq!(result, GeneratorResult::Stopped);
            let result = gen.for_each(|x| output.push(*x));
            assert_eq!(result, GeneratorResult::Complete);
            assert_eq!(output, [1, 2, 3]);
            let result = gen.for_each(|x| output.push(*x));
            assert_eq!(result, GeneratorResult::Complete);
            assert_eq!(output, [1, 2, 3]);
        }
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
        assert_eq!(output, [1, 2, 3, 4]);

        let result = generator.run(|x| {
            output.push(*x);
            ValueResult::MoreValues
        });

        assert_eq!(result, GeneratorResult::Complete);
        assert_eq!(output, [1, 2, 3, 4]);
    }

    #[test]
    fn take_while() {
        let data = [1, 2, 3, 4, 5];
        let mut output: Vec<i32> = Vec::new();

        let result = TakeWhile::new(SliceGenerator::new(&data), |x| **x <= 2).run(|x| {
            output.push(*x);
            ValueResult::MoreValues
        });
        assert_eq!(result, GeneratorResult::Complete);
        assert_eq!(output, [1, 2]);
    }

    #[test]
    fn take_while_with_restart() {
        let data = [1, 2, 3, 2, 2];
        let mut output: Vec<i32> = Vec::new();

        let mut gen = TakeWhile::new(SliceGenerator::new(&data), |x| **x <= 2);

        let result = gen.run(|x| {
            output.push(*x);
            ValueResult::MoreValues
        });
        assert_eq!(result, GeneratorResult::Complete);
        assert_eq!(output, [1, 2]);

        let result = gen.run(|x| {
            output.push(*x);
            ValueResult::MoreValues
        });
        assert_eq!(result, GeneratorResult::Complete);
        assert_eq!(output, [1, 2]);
    }

    #[test]
    fn spuriously_stopping_take_while() {
        let data = [1i32, 2, 3, 4, -1, 1, 2];
        for x in 0..5 {
            let mut gen = StoppingGen::new(x, &data).take_while(|x| x.is_positive());
            let mut output = Vec::new();
            let result = gen.for_each(|x| output.push(x));
            assert_eq!(result, GeneratorResult::Stopped);
            let result = gen.for_each(|x| output.push(x));
            assert_eq!(result, GeneratorResult::Complete);
            assert_eq!(output, [&1, &2, &3, &4]);
            let result = gen.for_each(|x| output.push(x));
            assert_eq!(result, GeneratorResult::Complete);
            assert_eq!(output, [&1, &2, &3, &4]);
        }
    }
}
