use crate::value::{ConsumerEmitter, ValueResult};
use std::marker::PhantomData;

pub struct Transform<Func, In, Out> {
    transform: Func,
    _phantom: PhantomData<(In, Out)>,
}

impl<Func, In, Out> Transform<Func, In, Out>
where
    Func: FnMut(In) -> Out,
{
    pub fn new(transform: Func) -> Self {
        Self {
            transform,
            _phantom: PhantomData,
        }
    }
}

impl<Func, In, Out> ConsumerEmitter for Transform<Func, In, Out>
where
    Func: FnMut(In) -> Out,
{
    type Input = In;
    type Output = Out;

    fn consume_and_emit(
        &mut self,
        value: Self::Input,
        mut output: impl FnMut(Self::Output) -> ValueResult,
    ) -> ValueResult {
        output((self.transform)(value))
    }
}
