use crate::ValueResult;
use crate::value::ConsumerEmitter;

use std::marker::PhantomData;

/// Filters values based on a predicate.
pub struct Filter<Pred, T> {
    predicate: Pred,
    _phantom: PhantomData<T>,
}

impl<Pred, T> Filter<Pred, T>
where
    Pred: FnMut(&T) -> bool,
{
    pub fn new(predicate: Pred) -> Self {
        Self {
            predicate,
            _phantom: PhantomData,
        }
    }
}

impl<Pred, T> ConsumerEmitter for Filter<Pred, T>
where
    Pred: FnMut(&T) -> bool,
{
    type Input = T;
    type Output = T;

    fn consume_and_emit(
        &mut self,
        value: Self::Input,
        mut output: impl FnMut(Self::Output) -> ValueResult,
    ) -> ValueResult {
        if (self.predicate)(&value) {
            output(value)
        } else {
            ValueResult::MoreValues
        }
    }
}
