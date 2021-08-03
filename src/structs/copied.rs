use crate::{Generator, GeneratorResult, ReverseGenerator, ValueResult};
use core::num::NonZeroUsize;

/// A generator that copies the elements of an underlying generator. See [`.copied()`](crate::GeneratorExt::copied) for details.
#[derive(Clone)]
pub struct Copied<Src> {
    source: Src,
}

impl<Src> Copied<Src> {
    pub(crate) fn new(source: Src) -> Self {
        Self { source }
    }
}

impl<'a, Src, T> Generator for Copied<Src>
where
    T: 'a + Copy,
    Src: Generator<Output = &'a T>,
{
    type Output = T;

    #[inline]
    fn run(&mut self, mut output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        self.source.run(|&x| output(x))
    }

    #[inline]
    fn try_advance(&mut self, n: core::num::NonZeroUsize) -> (usize, GeneratorResult) {
        self.source.try_advance(n)
    }
}

impl<'a, Src, T> ReverseGenerator for Copied<Src>
where
    T: 'a + Copy,
    Src: Generator<Output = &'a T> + ReverseGenerator,
{
    #[inline]
    fn run_back(&mut self, mut output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        self.source.run_back(|&x| output(x))
    }

    #[inline]
    fn try_advance_back(&mut self, n: NonZeroUsize) -> (usize, GeneratorResult) {
        self.source.try_advance_back(n)
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
        for x in 0..data.len() {
            let mut gen = StoppingGen::new(x as i32, &data).copied();
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
        let mut gen = SliceGenerator::new(&data).copied();
        assert_eq!(gen.next_back(), Ok(3));
        assert_eq!(gen.next_back(), Ok(2));
        assert_eq!(gen.next_back(), Ok(1));
        assert_eq!(gen.next_back(), Err(GeneratorResult::Complete));

        let data = [1, 2, 3];
        let mut gen = SliceGenerator::new(&data).copied();
        gen.try_advance_back(NonZeroUsize::new(1).unwrap());
        assert_eq!(gen.next_back(), Ok(2));
        assert_eq!(gen.next_back(), Ok(1));
        assert_eq!(gen.next_back(), Err(GeneratorResult::Complete));
    }
}
