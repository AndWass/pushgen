use crate::{Generator, GeneratorResult, ReverseGenerator, ValueResult};

/// Implements a filtered generator. See [`.filter()`](crate::GeneratorExt::filter) for more details.
#[derive(Clone)]
pub struct Filter<Gen, Pred> {
    generator: Gen,
    predicate: Pred,
}

impl<Gen, Pred> Filter<Gen, Pred>
where
    Gen: Generator,
    Pred: FnMut(&Gen::Output) -> bool,
{
    #[inline]
    pub(crate) fn new(generator: Gen, predicate: Pred) -> Self {
        Self {
            generator,
            predicate,
        }
    }
}

impl<Gen, Pred> Generator for Filter<Gen, Pred>
where
    Gen: Generator,
    Pred: FnMut(&Gen::Output) -> bool,
{
    type Output = Gen::Output;

    #[inline]
    fn run(&mut self, mut output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        let (generator, predicate) = (&mut self.generator, &mut self.predicate);
        generator.run(move |x| {
            if predicate(&x) {
                output(x)
            } else {
                ValueResult::MoreValues
            }
        })
    }
}

impl<Gen, Pred> ReverseGenerator for Filter<Gen, Pred>
where
    Gen: ReverseGenerator,
    Pred: FnMut(&Gen::Output) -> bool,
{
    #[inline]
    fn run_back(&mut self, mut output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        let (generator, predicate) = (&mut self.generator, &mut self.predicate);
        generator.run_back(move |x| {
            if predicate(&x) {
                output(x)
            } else {
                ValueResult::MoreValues
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::test::StoppingGen;
    use crate::{GeneratorExt, GeneratorResult, ReverseGenerator, SliceGenerator};
    use std::num::NonZeroUsize;

    #[test]
    fn spuriously_stopping() {
        let data = [1, 2, 3];
        fn is_odd(v: &&i32) -> bool {
            **v % 2 != 0
        }

        for x in 0..data.len() {
            let mut gen = StoppingGen::new(x as i32, &data).filter(is_odd);
            let mut output = Vec::new();
            let result = gen.for_each(|x| output.push(x));
            assert_eq!(result, GeneratorResult::Stopped);
            let result = gen.for_each(|x| output.push(x));
            assert_eq!(result, GeneratorResult::Complete);
            assert_eq!(output, [&1, &3]);
        }
    }

    #[test]
    fn reverse() {
        let data = [1, 2, 3];
        let mut gen = SliceGenerator::new(&data).filter(|x| *x % 2 == 1);
        assert_eq!(gen.next_back(), Ok(&3));
        assert_eq!(gen.next_back(), Ok(&1));
        assert_eq!(gen.next_back(), Err(GeneratorResult::Complete));

        let data = [1, 2, 3];
        let mut gen = SliceGenerator::new(&data).filter(|x| *x % 2 == 1);
        gen.try_advance_back(NonZeroUsize::new(1).unwrap());
        assert_eq!(gen.next_back(), Ok(&1));
        assert_eq!(gen.next_back(), Err(GeneratorResult::Complete));
    }
}
