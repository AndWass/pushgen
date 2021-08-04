use crate::{Generator, GeneratorResult, ReverseGenerator, ValueResult};
use core::num::NonZeroUsize;

/// Reverses a generators direction. See [`rev()`](crate::GeneratorExt::rev) for details.
pub struct Reverse<Src> {
    source: Src,
}

impl<Src> Reverse<Src> {
    pub(crate) fn new(source: Src) -> Self {
        Self { source }
    }
}

impl<Src> Generator for Reverse<Src>
where
    Src: ReverseGenerator,
{
    type Output = Src::Output;

    #[inline]
    fn run(&mut self, output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        self.source.run_back(output)
    }

    #[inline]
    fn try_advance(&mut self, n: NonZeroUsize) -> (usize, GeneratorResult) {
        self.source.try_advance_back(n)
    }
}

impl<Src> ReverseGenerator for Reverse<Src>
where
    Src: ReverseGenerator,
{
    #[inline]
    fn run_back(&mut self, output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        self.source.run(output)
    }

    #[inline]
    fn try_advance_back(&mut self, n: NonZeroUsize) -> (usize, GeneratorResult) {
        self.source.try_advance(n)
    }
}

#[cfg(test)]
mod tests {
    use crate::structs::Reverse;
    use crate::{GeneratorExt, GeneratorResult, SliceGenerator};

    #[test]
    fn reverse() {
        let data = [1, 2, 3, 4];
        let mut gen = Reverse::new(SliceGenerator::new(&data));

        assert_eq!(gen.next(), Ok(&4));
        assert_eq!(gen.next_back(), Ok(&1));
        assert_eq!(gen.next(), Ok(&3));
        assert_eq!(gen.next_back(), Ok(&2));

        assert_eq!(gen.next(), Err(GeneratorResult::Complete));
        assert_eq!(gen.next_back(), Err(GeneratorResult::Complete));
    }
}
