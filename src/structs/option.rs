use crate::{Generator, GeneratorResult, ValueResult};

/// A generator over the value in [`Some`] variant of an [`Option`].
///
/// The generator produces one value if the [`Option`] is a [`Some`], otherwise none.
///
/// This `struct` is created by the [`Option::into_gen`] function.
#[derive(Clone)]
pub struct OptionGen<T> {
    inner: Option<T>,
}

impl<T> OptionGen<T> {
    #[inline]
    pub(crate) fn new(inner: Option<T>) -> Self {
        Self { inner }
    }
}

impl<T> Generator for OptionGen<T> {
    type Output = T;

    #[inline]
    fn run(&mut self, mut output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        if let Some(v) = self.inner.take() {
            if output(v) == ValueResult::Stop {
                return GeneratorResult::Stopped;
            }
        }
        GeneratorResult::Complete
    }
}
