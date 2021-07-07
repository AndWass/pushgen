use crate::{Generator, GeneratorResult, ValueResult};

/// Creates a new generator where each iteration calls the provided closure
/// `F: FnMut() -> Option<T>`.
///
/// This allows creating a custom generator with any behavior
/// without using the more verbose syntax of creating a dedicated type
/// and implementing the [`Generator`] trait for it.
///
/// The closure can use captures and its environment to track state across iterations. Depending on
/// how the generator is used, this may require specifying the `move` keyword on the closure.
///
/// # Examples
///
/// Let’s re-implement the counter generator:
///
/// ```
/// use crate::pushgen::GeneratorExt;
///
/// let mut count = 0;
/// let mut counter = pushgen::from_fn(move || {
///     // Increment our count. This is why we started at zero.
///     count += 1;
///
///     // Check to see if we've finished counting or not.
///     if count < 6 {
///         Some(count)
///     } else {
///         None
///     }
/// });
/// let mut output: Vec<i32> = Vec::new();
/// counter.for_each(|x| output.push(x));
/// assert_eq!(output, [1, 2, 3, 4, 5]);
/// ```
#[inline]
pub fn from_fn<T, F>(f: F) -> FromFn<F>
where
    F: FnMut() -> Option<T>,
{
    FromFn(f)
}

/// A generator where each iteration calls the provided closure `F: FnMut() -> Option<T>`.
///
/// This `struct` is created by the [`from_fn()`] function.
/// See its documentation for more.
///
/// [`from_fn()`]: crate::from_fn
#[derive(Clone)]
pub struct FromFn<F>(F);

impl<T, F> Generator for FromFn<F>
where
    F: FnMut() -> Option<T>,
{
    type Output = T;

    #[inline]
    fn run(&mut self, mut output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        while let Some(v) = self.0() {
            if output(v) == ValueResult::Stop {
                return GeneratorResult::Stopped;
            }
        }
        GeneratorResult::Complete
    }
}
