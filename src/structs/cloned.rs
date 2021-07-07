use crate::{Generator, GeneratorResult, ValueResult};

/// A generator that clones the elements of an underlying generator. See `[.cloned()](crate::GeneratorExt::cloned)
/// for details
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

    fn run(&mut self, mut output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        self.source.run(|x| output(x.clone()))
    }
}
