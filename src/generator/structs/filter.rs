use crate::{Generator, ValueResult, GeneratorResult};

/// Create a filtered generator. Only values for which the predicate returns true will be passed on.
///
/// The predicate must implement `FnMut(&Gen::Output) -> bool`.
///
/// ## Example
/// ```
/// # use pipe_chan::*;
/// # use pipe_chan::generator::structs::Filter;
/// let input = [1,2,3,4];
/// let mut output: Vec<i32> = Vec::new();
/// let run_result = Filter::new(IteratorGenerator::new(input.iter()), |x| *x % 2 == 0).run(|x| {
///     output.push(*x);
///     ValueResult::MoreValues
/// });
/// assert_eq!(run_result, GeneratorResult::Complete);
/// assert_eq!(output, [2,4]);
/// ```
pub struct Filter<Gen, Pred>
{
    generator: Gen,
    predicate: Pred,
}

impl<Gen, Pred> Filter<Gen, Pred>
where
    Gen: Generator,
    Pred: FnMut(&Gen::Output) -> bool
{
    /// Create a new filtered generator from a source generator and a predicate.
    ///
    /// ## Example
    /// ```
    /// # use pipe_chan::*;
    /// # use pipe_chan::generator::structs::Filter;
    /// let input = [1,2,3,4];
    /// let even_value_filter = Filter::new(IteratorGenerator::new(input.iter()), |x| *x % 2 == 0);
    /// ```
    #[inline]
    pub fn new(generator: Gen, predicate: Pred) -> Self {
        Self {
            generator,
            predicate
        }
    }
}

impl<Gen, Pred> Generator for Filter<Gen, Pred>
where
    Gen: Generator,
    Pred: FnMut(&Gen::Output) -> bool
{
    type Output = Gen::Output;

    #[inline]
    fn run(&mut self, mut output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        let (generator, predicate) = (&mut self.generator, &mut self.predicate);
        generator.run(move |x| {
            if predicate(&x) {
                output(x)
            }
            else {
                ValueResult::MoreValues
            }
        })
    }
}