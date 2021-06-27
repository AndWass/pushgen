use crate::generator::structs::{Chain, Filter, Skip, Take, Transform};
use crate::{Generator, GeneratorResult, ValueResult};

pub trait Sealed {}

impl<T> Sealed for T where T: Generator {}

pub trait GeneratorExt: Sealed + Generator {
    fn chain<Gen>(self, other: Gen) -> Chain<Self, Gen>
    where
        Self: Sized,
        Gen: Generator<Output = Self::Output>,
    {
        Chain::new(self, other)
    }

    fn filter<Pred>(self, predicate: Pred) -> Filter<Self, Pred>
    where
        Self: Sized,
        Pred: FnMut(&Self::Output) -> bool,
    {
        Filter::new(self, predicate)
    }

    fn transform<Trans, Out>(self, transform_fn: Trans) -> Transform<Self, Trans, Out>
    where
        Self: Sized,
        Trans: FnMut(Self::Output) -> Out,
    {
        Transform::new(self, transform_fn)
    }

    fn skip(self, amount: usize) -> Skip<Self>
    where
        Self: Sized,
    {
        Skip::new(self, amount)
    }

    fn take(self, amount: usize) -> Take<Self>
    where
        Self: Sized,
    {
        Take::new(self, amount)
    }

    /// Run a generator to completion (or until it is stopped) and apply a function for each value
    /// produced by the generator.
    ///
    /// The closure will be called for as long as the generator produces values, it is not possible
    /// to abort processing early. If early break is needed, use [`Generator::run`](crate::Generator::run)
    /// ## Example
    /// ```
    /// # use pipe_chan::generator::SliceGenerator;
    /// # use pipe_chan::{GeneratorExt, GeneratorResult};
    /// let mut sum = 0i32;
    /// let data = [1,2,3];
    /// let result = SliceGenerator::new(&data).for_each(|x| sum += x);
    /// assert_eq!(sum, 6);
    /// assert_eq!(result, GeneratorResult::Complete);
    /// ```
    #[inline]
    fn for_each<Func>(&mut self, mut func: Func) -> GeneratorResult
    where
        Self: Sized,
        Func: FnMut(Self::Output)
    {
        self.run(move |value| {
            func(value);
            ValueResult::MoreValues
        })
    }
}

impl<T: Generator> GeneratorExt for T {}

#[cfg(test)]
mod tests
{
    use crate::{Generator, ValueResult, GeneratorResult, GeneratorExt};

    #[test]
    fn for_each_stopped()
    {
        struct StoppingGen;
        impl Generator for StoppingGen
        {
            type Output = i32;

            fn run(&mut self, _output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
                GeneratorResult::Stopped
            }
        }

        let mut gen = StoppingGen;
        assert_eq!(gen.for_each(|_| ()), GeneratorResult::Stopped);
    }
}
