use crate::{GeneratorResult, ValueResult};
use either::Either;

/// Trait for generating values into a closure.
///
/// When a `Generator` is [`run()`](crate::Generator::run) it generates values that are fed an `output` closure.
/// It continues to feed values to the closure for as long as it can, unless the closure returns
/// [`ValueResult::Stop`](crate::ValueResult::Stop).
///
/// When all values have been generated the `run()` method returns [`GeneratorResult::Complete`](crate::GeneratorResult::Complete).
/// If `output` returns [`ValueResult::Stop`](crate::ValueResult::Stop) for any value
/// the generator must not call `output` with any further values and return [`GeneratorResult::Stopped`](crate::GeneratorResult::Stopped)
/// as well.
///
/// **The generator must not assume that it won't be called again after it returns**.
///
/// ## Example
///
/// A generic generator can be written like this:
/// ```
/// use pushgen::{Generator, ValueResult, GeneratorResult};
/// struct GenericGenerator<Out, Gen>
/// where
///     Gen: FnMut() -> Option<Out>,
/// {
///     generator: Gen,
/// }
///
/// impl<Out, Gen> Generator for GenericGenerator<Out, Gen>
///     where
///         Gen: FnMut() -> Option<Out>,
/// {
///     type Output = Out;
///
///     fn run(&mut self, mut output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
///         while let Some(value) = (self.generator)() {
///             if output(value) == ValueResult::Stop {
///                 return GeneratorResult::Stopped;
///             }
///         }
///         GeneratorResult::Complete
///     }
/// }
/// ```
pub trait Generator {
    /// Data-type generated by the generator.
    type Output;

    /// Run the generator, emitting values to the `output` closure. New values are emitted for
    /// as long as the closure returns [`ValueResult::MoreValues`](crate::ValueResult::MoreValues).
    /// If the closure returns [`ValueResult::Stop`](crate::ValueResult::Stop) the generator **must**
    /// return [`GeneratorResult::Stopped`](crate::GeneratorResult::Stopped).
    fn run(&mut self, output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult;
}

impl<L, R> Generator for Either<L, R>
where
    L: Generator,
    R: Generator<Output = L::Output>,
{
    type Output = L::Output;

    #[inline]
    fn run(&mut self, output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        match self {
            Either::Left(left) => left.run(output),
            Either::Right(right) => right.run(output),
        }
    }
}
