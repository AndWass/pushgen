
pub trait InputStage {
    type Input;
    fn process(&mut self, value: Self::Input) -> bool;
}

pub trait InputOutputStage: InputStage {
    type Output;
    fn process_and_then(&mut self, value: Self::Input, next: &mut dyn InputStage<Input = Self::Output>) -> bool;
}
