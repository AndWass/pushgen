use crate::{Generator, GeneratorResult, ValueResult};

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
}
