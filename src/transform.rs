use crate::{InputStage, InputOutputStage};
use std::marker::PhantomData;

pub struct Transform<From, To, F: FnMut(&From) -> To> {
    transform: F,
    _phantom: PhantomData<(From, To)>,
}

impl<From, To, F: FnMut(&From) -> To> Transform<From, To, F> {
    #[inline(always)]
    pub fn new(transform: F) -> Self {
        Self {
            transform,
            _phantom: PhantomData,
        }
    }
}

impl<From, To, F: FnMut(&From) -> To> InputStage for Transform<From, To, F> {
    type Input = From;

    #[inline(always)]
    fn process(&mut self, value: Self::Input) -> bool {
        (self.transform)(&value);
        true
    }
}

impl<From, To, F: FnMut(&From) -> To> InputOutputStage for Transform<From, To, F> {
    type Output = To;

    #[inline(always)]
    fn process_and_then(&mut self, value: Self::Input, next: &mut dyn InputStage<Input=Self::Output>) -> bool {
        next.process((self.transform)(&value))
    }
}
