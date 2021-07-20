use crate::{traits::DynGenerator, Generator, GeneratorResult, ValueResult};

/// Box a generator, type-erasing the actual generator type.
/// See [`.boxed()`](crate::GeneratorExt::boxed) for details.
pub struct BoxedGenerator<T> {
    source: Box<dyn DynGenerator<Output = T>>,
}

impl<T> BoxedGenerator<T> {
    #[inline]
    pub(crate) fn new(source: impl DynGenerator<Output = T> + 'static) -> Self {
        Self {
            source: Box::new(source),
        }
    }
}

impl<T> Generator for BoxedGenerator<T> {
    type Output = T;

    #[inline]
    fn run(&mut self, mut output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        self.source.as_mut().run_dyn(&mut output)
    }
}
