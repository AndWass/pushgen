use crate::traits::generator_ext::Sealed;
use crate::{Generator, GeneratorResult, ValueResult};

/// Sealed trait to enable boxed generators. See [`.boxed()`](crate::GeneratorExt::boxed) for details.
///
/// This trait should normally not be used. Pretty much the only use-case for this is to be an object-safe
/// trait, thus allowing for dynamic trait objects and boxing.
///
/// This trait is blanked implemented for all generators.
pub trait DynGenerator: Sealed {
    /// The output type of this generator.
    type Output;
    /// Run the generator using a `&mut dyn FnMut` instead of `impl FnMut`. This
    fn run_dyn(&mut self, output: &mut dyn FnMut(Self::Output) -> ValueResult) -> GeneratorResult;
}

impl<T> DynGenerator for T
where
    T: Generator,
{
    type Output = T::Output;

    #[inline]
    fn run_dyn(&mut self, output: &mut dyn FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        self.run(output)
    }
}
