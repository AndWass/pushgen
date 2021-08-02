use crate::{Generator, GeneratorResult, ValueResult};
use core::num::NonZeroUsize;

/// A generator that yields the current count and the value when run. See [`enumerate()`](crate::GeneratorExt::enumerate) for details.
pub struct Enumerate<Src> {
    source: Src,
    index: usize,
}

impl<Src> Enumerate<Src> {
    #[inline]
    pub(crate) fn new(source: Src) -> Self {
        Self { source, index: 0 }
    }
}

impl<Src> Generator for Enumerate<Src>
where
    Src: Generator,
{
    type Output = (usize, Src::Output);

    #[inline]
    fn run(&mut self, mut output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        let index = &mut self.index;
        self.source.run(|x| {
            let res = output((*index, x));
            *index += 1;
            res
        })
    }

    #[inline]
    fn try_advance(&mut self, n: NonZeroUsize) -> (usize, GeneratorResult) {
        let res = self.source.try_advance(n);
        self.index += res.0;
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::test::StoppingGen;
    use crate::{Generator, GeneratorExt, GeneratorResult, SliceGenerator};
    use std::num::NonZeroUsize;

    #[test]
    fn enumerate() {
        let data = ['a', 'b', 'c'];

        let mut gen = SliceGenerator::new(&data).enumerate();
        assert_eq!(gen.next(), Ok((0, &'a')));
        assert_eq!(gen.next(), Ok((1, &'b')));
        assert_eq!(gen.next(), Ok((2, &'c')));
        assert_eq!(gen.next(), Err(GeneratorResult::Complete));
    }

    #[test]
    fn advance() {
        let data = ['a', 'b', 'c'];

        let mut gen = SliceGenerator::new(&data).enumerate();
        gen.try_advance(NonZeroUsize::new(2).unwrap());
        assert_eq!(gen.next(), Ok((2, &'c')));
        assert_eq!(gen.next(), Err(GeneratorResult::Complete));
    }

    #[test]
    fn spuriously_stopping_advance() {
        let data = ['a', 'b', 'c'];

        let mut gen = StoppingGen::new(1, &data).enumerate();
        gen.try_advance(NonZeroUsize::new(2).unwrap());
        gen.try_advance(NonZeroUsize::new(1).unwrap());
        assert_eq!(gen.next(), Ok((2, &'c')));
        assert_eq!(gen.next(), Err(GeneratorResult::Complete));
    }
}
