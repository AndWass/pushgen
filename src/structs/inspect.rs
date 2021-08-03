use crate::{Generator, GeneratorResult, ReverseGenerator, ValueResult};
use core::num::NonZeroUsize;

/// Inspect each value and then pass it on. See [`inspect()`](crate::GeneratorExt::inspect) for details.
pub struct Inspect<Src, F> {
    source: Src,
    inspector: F,
}

impl<Src, F> Inspect<Src, F> {
    pub(crate) fn new(source: Src, inspector: F) -> Self {
        Self { source, inspector }
    }
}

impl<Src, F> Generator for Inspect<Src, F>
where
    Src: Generator,
    F: FnMut(&Src::Output),
{
    type Output = Src::Output;

    #[inline]
    fn run(&mut self, mut output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        let inspector = &mut self.inspector;
        self.source.run(move |x| {
            inspector(&x);
            output(x)
        })
    }

    #[inline]
    fn try_advance(&mut self, n: NonZeroUsize) -> (usize, GeneratorResult) {
        self.source.try_advance(n)
    }
}

impl<Src, F> ReverseGenerator for Inspect<Src, F>
where
    Src: ReverseGenerator,
    F: FnMut(&Src::Output),
{
    #[inline]
    fn run_back(&mut self, mut output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        let inspector = &mut self.inspector;
        self.source.run_back(move |x| {
            inspector(&x);
            output(x)
        })
    }

    #[inline]
    fn try_advance_back(&mut self, n: NonZeroUsize) -> (usize, GeneratorResult) {
        self.source.try_advance_back(n)
    }
}

#[cfg(test)]
mod tests {
    use crate::{GeneratorExt, ReverseGenerator, SliceGenerator, ValueResult};
    use std::num::NonZeroUsize;

    #[test]
    fn inspect() {
        let a = [1, 4, 2, 3];
        let mut before_filter = Vec::new();
        let mut after_filter = Vec::new();

        let sum: i32 = SliceGenerator::new(&a)
            .cloned()
            .inspect(|x| before_filter.push(*x))
            .filter(|x| x % 2 == 0)
            .inspect(|x| after_filter.push(*x))
            .sum();

        assert_eq!(sum, 6);
        assert_eq!(before_filter, [1, 4, 2, 3]);
        assert_eq!(after_filter, [4, 2]);
    }

    #[test]
    fn reverse() {
        let a = [1, 4, 2, 3];
        let mut before_filter = Vec::new();
        let mut after_filter = Vec::new();

        let mut gen = SliceGenerator::new(&a)
            .cloned()
            .inspect(|x| before_filter.push(*x))
            .filter(|x| x % 2 == 0)
            .inspect(|x| after_filter.push(*x));

        gen.run_back(|_| ValueResult::MoreValues);

        assert_eq!(before_filter, [3, 2, 4, 1]);
        assert_eq!(after_filter, [2, 4]);

        let mut before_filter = Vec::new();
        let mut after_filter = Vec::new();

        let mut gen = SliceGenerator::new(&a)
            .cloned()
            .inspect(|x| before_filter.push(*x))
            .filter(|x| x % 2 == 0)
            .inspect(|x| after_filter.push(*x));

        gen.try_advance_back(NonZeroUsize::new(1).unwrap());
        gen.run_back(|_| ValueResult::MoreValues);

        assert_eq!(before_filter, [3, 2, 4, 1]);
        assert_eq!(after_filter, [4]);
    }
}
