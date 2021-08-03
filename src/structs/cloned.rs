use crate::{Generator, GeneratorResult, ValueResult, ReverseGenerator};
use core::num::NonZeroUsize;

/// A generator that clones the elements of an underlying generator. See `[.cloned()](crate::GeneratorExt::cloned)
/// for details
#[derive(Clone)]
pub struct Cloned<Src> {
    source: Src,
}

impl<Src> Cloned<Src> {
    #[inline]
    pub(crate) fn new(source: Src) -> Self {
        Self { source }
    }
}

impl<'a, Src, T> Generator for Cloned<Src>
where
    T: 'a + Clone,
    Src: Generator<Output = &'a T>,
{
    type Output = T;

    #[inline]
    fn run(&mut self, mut output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        self.source.run(|x| output(x.clone()))
    }

    #[inline]
    fn try_advance(&mut self, n: core::num::NonZeroUsize) -> (usize, GeneratorResult) {
        self.source.try_advance(n)
    }
}

impl<'a, Src, T> ReverseGenerator for Cloned<Src>
where
    T: 'a + Clone,
    Src: Generator<Output = &'a T> + ReverseGenerator,
{
    #[inline]
    fn run_back(&mut self, mut output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        self.source.run_back(|x| output(x.clone()))
    }

    #[inline]
    fn try_advance_back(&mut self, n: NonZeroUsize) -> (usize, GeneratorResult) {
        self.source.try_advance_back(n)
    }
}

#[cfg(test)]
mod tests {
    use crate::test::StoppingGen;
    use crate::{GeneratorExt, GeneratorResult, SliceGenerator, ReverseGenerator};
    use std::num::NonZeroUsize;

    #[test]
    fn spuriously_stopping() {
        let data = [1, 2, 3];
        for x in 0..data.len() {
            let mut gen = StoppingGen::new(x as i32, &data).cloned();
            let mut output: Vec<i32> = Vec::new();
            let result = gen.for_each(|x| output.push(x));
            assert_eq!(result, GeneratorResult::Stopped);
            let result = gen.for_each(|x| output.push(x));
            assert_eq!(result, GeneratorResult::Complete);
            assert_eq!(output, [1, 2, 3]);
        }
    }

    #[test]
    fn reverse() {
        let data = [1, 2, 3];
        let mut gen = SliceGenerator::new(&data).cloned();
        assert_eq!(gen.next_back(), Ok(3));
        assert_eq!(gen.next_back(), Ok(2));
        assert_eq!(gen.next_back(), Ok(1));
        assert_eq!(gen.next_back(), Err(GeneratorResult::Complete));

        let data = [1, 2, 3];
        let mut gen = SliceGenerator::new(&data).cloned();
        gen.try_advance_back(NonZeroUsize::new(1).unwrap());
        assert_eq!(gen.next_back(), Ok(2));
        assert_eq!(gen.next_back(), Ok(1));
        assert_eq!(gen.next_back(), Err(GeneratorResult::Complete));
    }
}
