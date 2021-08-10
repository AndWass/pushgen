use crate::{Generator, GeneratorResult, ValueResult};

/// Repeates a generator endlessly. See [`cycle()`](crate::GeneratorExt::cycle) for details.
pub struct Cycle<Src> {
    source: Src,
    current: Src,
}

impl<Src: Clone> Cycle<Src> {
    pub(crate) fn new(source: Src) -> Self {
        Self {
            source: source.clone(),
            current: source,
        }
    }
}

impl<Src: Clone + Generator> Generator for Cycle<Src> {
    type Output = Src::Output;

    #[inline]
    fn run(&mut self, mut output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        loop {
            match self.current.run(&mut output) {
                GeneratorResult::Stopped => return GeneratorResult::Stopped,
                GeneratorResult::Complete => self.current = self.source.clone(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::MultiStoppingGen;
    use crate::{GeneratorExt, IntoGenerator};

    #[test]
    fn cycle() {
        let data = [1, 2, 3];
        let mut gen = (&data).into_gen().cycle();

        assert_eq!(gen.next(), Ok(&1));
        assert_eq!(gen.next(), Ok(&2));
        assert_eq!(gen.next(), Ok(&3));

        assert_eq!(gen.next(), Ok(&1));
        assert_eq!(gen.next(), Ok(&2));
        assert_eq!(gen.next(), Ok(&3));

        assert_eq!(gen.next(), Ok(&1));
        assert_eq!(gen.next(), Ok(&2));
        assert_eq!(gen.next(), Ok(&3));
    }

    #[test]
    fn spuriously_stopping() {
        use crate::GeneratorResult::Stopped;
        let data = [None, None, Some(1), None, Some(2), None];
        let mut gen = Cycle::new(MultiStoppingGen::new(&data));

        assert_eq!(gen.next(), Err(Stopped));
        assert_eq!(gen.next(), Err(Stopped));
        assert_eq!(gen.next(), Ok(&1));
        assert_eq!(gen.next(), Err(Stopped));
        assert_eq!(gen.next(), Ok(&2));
        assert_eq!(gen.next(), Err(Stopped));

        assert_eq!(gen.next(), Err(Stopped));
        assert_eq!(gen.next(), Err(Stopped));
        assert_eq!(gen.next(), Ok(&1));
        assert_eq!(gen.next(), Err(Stopped));
        assert_eq!(gen.next(), Ok(&2));
        assert_eq!(gen.next(), Err(Stopped));

        assert_eq!(gen.next(), Err(Stopped));
        assert_eq!(gen.next(), Err(Stopped));
        assert_eq!(gen.next(), Ok(&1));
        assert_eq!(gen.next(), Err(Stopped));
        assert_eq!(gen.next(), Ok(&2));
        assert_eq!(gen.next(), Err(Stopped));

        assert_eq!(gen.next(), Err(Stopped));
        assert_eq!(gen.next(), Err(Stopped));
        assert_eq!(gen.next(), Ok(&1));
        assert_eq!(gen.next(), Err(Stopped));
        assert_eq!(gen.next(), Ok(&2));
        assert_eq!(gen.next(), Err(Stopped));
    }
}
