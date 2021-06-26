use std::marker::PhantomData;
use crate::{Generator, ValueResult, GeneratorResult};

pub struct Transform<Gen, Func, Out>
{
    source: Gen,
    transform: Func,
    _phantom: PhantomData<Out>
}

impl<Gen, Func, Out> Transform<Gen, Func, Out>
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

impl<Gen, Func, Out> Generator for Transform<Gen, Func, Out>
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
