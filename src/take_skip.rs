use std::marker::PhantomData;
use crate::{InputStage, InputOutputStage};

pub struct Take<T> {
    amount: usize,
    _phantom: PhantomData<T>,
}

impl<T> Take<T> {
    pub fn new(amount: usize) -> Self {
        Self {
            amount,
            _phantom: PhantomData
        }
    }
}

impl<T> InputStage for Take<T> {
    type Input = T;

    #[inline(always)]
    fn process(&mut self, _value: Self::Input) -> bool {
        if self.amount > 0 {
            self.amount -= 1;
            true
        }
        else {
            false
        }
    }
}

impl<T> InputOutputStage for Take<T> {
    type Output = T;

    #[inline(always)]
    fn process_and_then(&mut self, value: Self::Input, next: &mut dyn InputStage<Input=Self::Output>) -> bool {
        if self.amount > 0 {
            self.amount -= 1;
            next.process(value)
        }
        else {
            false
        }
    }
}

pub struct Skip<T> {
    amount: usize,
    _phantom: PhantomData<T>,
}

impl<T> Skip<T> {
    pub fn new(amount: usize) -> Self {
        Self {
            amount,
            _phantom: PhantomData
        }
    }
}

impl<T> InputStage for Skip<T> {
    type Input = T;
    #[inline(always)]
    fn process(&mut self, _value: Self::Input) -> bool {
        if self.amount > 0 {
            self.amount -= 1;
        }
        true
    }
}

impl<T> InputOutputStage for Skip<T> {
    type Output = T;

    #[inline(always)]
    fn process_and_then(&mut self, value: Self::Input, next: &mut dyn InputStage<Input=Self::Output>) -> bool {
        if self.amount > 0 {
            self.amount -= 1;
            true
        }
        else {
            next.process(value)
        }
    }
}
