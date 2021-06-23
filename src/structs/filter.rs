use std::marker::PhantomData;

use crate::{InputStage, InputOutputStage};

/// Filters values using a closure predicate. See [`.filter()`](crate::StageExt::filter) for
/// more information.
pub struct Filter<F, A>
where
    F: FnMut(&A) -> bool,
{
    predicate: F,
    _phantom: PhantomData<A>,
}

impl<F, A> Filter<F, A>
where
    F: FnMut(&A) -> bool,
{
    pub fn new(predicate: F) -> Self {
        Self {
            predicate,
            _phantom: PhantomData,
        }
    }
}

impl<F, A> InputStage for Filter<F, A>
where F: FnMut(&A) -> bool {
    type Input = A;

    fn process(&mut self, value: Self::Input) -> bool {
        (self.predicate)(&value);
        true
    }
}

impl<F, A> InputOutputStage for Filter<F, A>
where F: FnMut(&A) -> bool {
    type Output = A;

    #[inline]
    fn process_and_then(&mut self, value: Self::Input, next: &mut dyn InputStage<Input=Self::Output>) -> bool {
        if (self.predicate)(&value) {
            next.process(value)
        }
        else {
            true
        }
    }
}