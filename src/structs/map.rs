use core::marker::PhantomData;
use crate::{Generator, ValueResult, GeneratorResult};

/// Implements a mapped generator. See [`.map()`](crate::GeneratorExt::map) for details.
pub struct Map<Gen, Func, Out>
{
    source: Gen,
    transform: Func,
    _phantom: PhantomData<Out>
}

impl<Gen, Func, Out> Map<Gen, Func, Out>
where
    Gen: Generator,
    Func: FnMut(Gen::Output) -> Out
{
    #[inline]
    pub fn new(source: Gen, transform: Func) -> Self {
        Self {
            source,
            transform,
            _phantom: PhantomData
        }
    }
}

impl<Gen, Func, Out> Generator for Map<Gen, Func, Out>
where
    Gen: Generator,
    Func: FnMut(Gen::Output) -> Out {
    type Output = Out;

    #[inline]
    fn run(&mut self, mut output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        let transform = &mut self.transform;
        self.source.run(move |value| output(transform(value)))
    }
}
