use crate::{Generator, GeneratorResult, ValueResult};
use std::num::NonZeroUsize;

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
        self.source.run(|x| {
            inspector(&x);
            output(x)
        })
    }

    #[inline]
    fn try_advance(&mut self, n: NonZeroUsize) -> (usize, GeneratorResult) {
        self.source.try_advance(n)
    }
}

#[cfg(test)]
mod tests {
    use crate::{GeneratorExt, SliceGenerator};

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
}
