pub mod structs;

use std::marker::PhantomData;
use crate::ValueResult;

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
#[repr(u8)]
pub enum GeneratorResult {
    /// Returned from `Generator::run` when the generator was stopped because the `output` function
    /// returned `ValueResult::Stop`
    Stopped,
    /// Returned from `Generator::run` when the generator has sent all values to the `output` function.
    /// When this has been returned the generator will never generate more values again.
    Complete
}

impl From<bool> for GeneratorResult {
    fn from(b: bool) -> Self {
        if !b {
            Self::Stopped
        }
        else {
            Self::Complete
        }
    }
}

/// When a `Generator` is `.run()` it generates values that are fed to the supplied `output` closure.
/// It continues to feed values to the output closure for as long as it can and calls `output` for
/// every value generated.
///
/// When all values have been generated it returns `true`. If `output` returns false for any value
/// the generator must stop generating new values and immediately return `false` as well.
///
/// **The generator must not assume that it won't be called again after it returns**.
///
/// ## Example
///
/// A generic generator can be written like this:
/// ```
/// use pipe_chan::{Generator, ValueResult, GeneratorResult};
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
    type Output;

    /// Run the generator, emitting values to the `output` closure. New values are emitted for
    /// as long as the closure returns `true`. If the closure returns `false` the generator **must**
    /// return `false`.
    fn run(&mut self, output: impl FnMut(Self::Output) -> crate::ValueResult) -> GeneratorResult;
}

/// A generic generator that adapts a closure as a generator. The closure must have the form
/// `FnMut() -> Option<T>` and the generator will have `Generator::Output=T`.
pub struct GenericGenerator<Out, Gen> {
    generator: Gen,
    _phantom: PhantomData<Out>,
}

impl<Out, Gen> GenericGenerator<Out, Gen>
    where
        Gen: FnMut() -> Option<Out>,
{
    /// Create a new GenericGenerator.
    ///
    /// ## Example
    /// ```
    /// # use pipe_chan::GenericGenerator;
    /// let always42 = GenericGenerator::new(|| Some(42));
    /// ```
    pub fn new(generator: Gen) -> Self {
        Self {
            generator,
            _phantom: PhantomData,
        }
    }
}

impl<Out, Gen> Generator for GenericGenerator<Out, Gen>
    where
        Gen: FnMut() -> Option<Out>,
{
    type Output = Out;

    fn run(&mut self, mut output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        while let Some(value) = (self.generator)() {
            if output(value) == ValueResult::Stop {
                return GeneratorResult::Stopped;
            }
        }
        GeneratorResult::Complete
    }
}
