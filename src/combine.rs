use crate::{Generator, GeneratorResult};
use crate::value::{ConsumerEmitter, ValueResult};
/// Combines either a [`Generator`](crate::stream::Generator) or a [`ConsumerEmitter`](crate::stream::ConsumerEmitter)
/// with another [`ConsumerEmitter`](crate::stream::ConsumerEmitter).
///
/// `Combine<Generator, ConsumerEmitter>` implements [`Generator`](crate::stream::Generator), while `Combine<ConsumerEmitter, ConsumerEmitter>`
/// implements [`ConsumerEmitter`](crate::stream::ConsumerEmitter).
pub struct Combine<First, Second> {
    first: First,
    second: Second,
}

impl<First, Second> Combine<First, Second>
{
    pub fn new(first: First, second: Second) -> Self {
        Self {
            first,
            second
        }
    }
}

impl<First, Second> Generator for Combine<First, Second>
where
    First: Generator,
    Second: ConsumerEmitter<Input = First::Output>,
{
    type Output = Second::Output;
    #[inline]
    fn run(&mut self, mut output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        let (first, second) = (&mut self.first, &mut self.second);
        first.run(move |value| second.consume_and_emit(value, &mut output))
    }
}

impl<First, Second> ConsumerEmitter for Combine<First, Second>
where
    First: ConsumerEmitter,
    Second: ConsumerEmitter<Input = First::Output>,
{
    type Input = First::Input;
    type Output = Second::Output;
    #[inline]
    fn consume_and_emit(
        &mut self,
        value: Self::Input,
        mut output: impl FnMut(Self::Output) -> ValueResult,
    ) -> ValueResult {
        let (first, second) = (&mut self.first, &mut self.second);
        first.consume_and_emit(value, move |second_value| {
            second.consume_and_emit(second_value, &mut output)
        })
    }
}
