use crate::{InputOutputStage, InputStage};


struct ThenRef<'a, T>
where T: InputOutputStage
{
    first: &'a mut T,
    second: &'a mut dyn InputStage<Input = T::Output>,
}

impl<'a, T> InputStage for ThenRef<'a, T>
where
    T: InputOutputStage,
{
    type Input = T::Input;
    #[inline(always)]
    fn process(&mut self, value: Self::Input) -> bool {
        self.first.process_and_then(value, self.second)
    }
}

/// Combines two stages, where the first must implement [`InputOutputStage`](crate::InputOutputStage).
pub struct Then<T, U> {
    first: T,
    second: U,
}

impl<T, U> Then<T, U> {
    pub fn new(first: T, second: U) -> Self {
        Self { first, second }
    }
}

impl<T, U> InputStage for Then<T, U>
where
    T: InputOutputStage,
    U: InputStage<Input = T::Output>,
{
    type Input = T::Input;
    #[inline(always)]
    fn process(&mut self, value: Self::Input) -> bool {
        self.first.process_and_then(value, &mut self.second)
    }
}

impl<T, U> InputOutputStage for Then<T, U>
where
    T: InputOutputStage,
    U: InputOutputStage<Input = T::Output>,
{
    type Output = U::Output;
    #[inline(always)]
    fn process_and_then(
        &mut self,
        value: Self::Input,
        next: &mut dyn InputStage<Input = Self::Output>,
    ) -> bool {
        let mut then_ref = ThenRef {
            first: &mut self.second,
            second: next,
        };
        self.first.process_and_then(value, &mut then_ref)
    }
}
