use std::marker::PhantomData;
use crate::{InputStage, InputOutputStage};

/// An input-output stage that just forwards values to the next staqe. See [`begin()`](crate::begin)
/// for more details.
pub struct Begin<T> {
    _phantom: PhantomData<T>,
}

impl<T> Begin<T> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<T> InputStage for Begin<T> {
    type Input = T;

    #[inline(always)]
    fn process(&mut self, _value: Self::Input) -> bool {
        true
    }
}

impl<T> InputOutputStage for Begin<T> {
    type Output = T;

    #[inline(always)]
    fn process_and_then(
        &mut self,
        value: Self::Input,
        next: &mut dyn InputStage<Input = Self::Output>,
    ) -> bool {
        next.process(value)
    }
}

/// An end-stage that calls a closure for each value produced. See [`.end()`](crate::StageExt::end)
/// for more details.
pub struct End<F, A> {
    consumer: F,
    _phantom: PhantomData<A>,
}

impl<F, A, R> End<F, A>
    where
        F: FnMut(A) -> R,
{
    pub fn new(consumer: F) -> Self {
        Self {
            consumer,
            _phantom: PhantomData,
        }
    }
}

impl<F, A> InputStage for End<F, A>
    where
        F: FnMut(A) -> bool,
{
    type Input = A;
    #[inline(always)]
    fn process(&mut self, value: Self::Input) -> bool {
        (self.consumer)(value)
    }
}

/// A variant of [`.end()`](crate::StageExt::end) that always returns true. See [`.collect()`](crate::StageExt::collect)
/// for details.
pub struct Collect<Func, Arg> {
    collector: Func,
    _phantom: PhantomData<Arg>,
}

impl<Func, Arg> Collect<Func, Arg>
where
    Func: FnMut(Arg) {
    pub fn new(collector: Func) -> Self {
        Self {
            collector,
            _phantom: PhantomData
        }
    }
}

impl<Func, Arg> InputStage for Collect<Func, Arg>
where
    Func: FnMut(Arg) {
    type Input = Arg;

    #[inline(always)]
    fn process(&mut self, value: Self::Input) -> bool {
        (self.collector)(value);
        true
    }
}
