use crate::{Generator, GeneratorResult, ValueResult};

/// Creates a generator that wraps an `Iterator`.
///
/// Prefer [`Generator`]s where possible to ensure minimal overhead.  See [`IntoGenerator`].
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use crate::pushgen::GeneratorExt;
/// let v = vec![1, 2, 3];
/// let mut gen = pushgen::from_iter(v);
///
/// let mut output: Vec<i32> = Vec::new();
/// gen.for_each(|x| output.push(x));
/// assert_eq!(output, [1, 2, 3]);
/// ```
///
/// [`Generator`]: crate::Generator
/// [`IntoGenerator`]: crate::IntoGenerator
#[inline]
pub fn from_iter<I: IntoIterator>(iterable: I) -> FromIter<I::IntoIter> {
    FromIter(iterable.into_iter())
}

/// A generator where each iteration delegates to an `Iterator`
///
/// This `struct` is created by the [`from_iter()`] function.
/// See its documentation for more.
///
/// [`from_iter()`]: crate::from_iter
#[derive(Clone)]
pub struct FromIter<I>(I);

impl<I: Iterator> Generator for FromIter<I> {
    type Output = I::Item;

    #[inline]
    fn run(&mut self, mut output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        while let Some(v) = self.0.next() {
            if output(v) == ValueResult::Stop {
                return GeneratorResult::Stopped;
            }
        }
        GeneratorResult::Complete
    }
}
