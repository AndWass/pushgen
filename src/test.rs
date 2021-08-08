//! Various test utilities
//!
//! These are available when the feature `test` is enabled (**disabled** by default), and can  be used
//! to test generator adaptors.

use crate::{Generator, GeneratorResult, SliceGenerator, ValueResult};

/// A spuriously stopping generator that will stop once.
pub struct StoppingGen<'a, T> {
    stop_at: i32,
    stopped_data: Option<&'a T>,
    data: SliceGenerator<'a, T>,
}

impl<'a, T> StoppingGen<'a, T> {
    /// Create a new stopping generator.
    ///
    /// ## Example
    ///
    /// ```
    /// use pushgen::test::StoppingGen;
    /// use pushgen::{GeneratorResult, GeneratorExt};
    /// let data = [1, 2, 3];
    /// let mut gen = StoppingGen::new(0, &data);
    /// assert_eq!(gen.next(), Err(GeneratorResult::Stopped));
    /// ```
    pub fn new(stop_at: i32, data: &'a [T]) -> Self {
        Self {
            stop_at,
            stopped_data: None,
            data: SliceGenerator::new(data),
        }
    }
}

impl<'a, T> Generator for StoppingGen<'a, T> {
    type Output = &'a T;

    fn run(&mut self, mut output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        if self.stop_at == 0 {
            self.stop_at -= 1;
            return GeneratorResult::Stopped;
        }

        if let Some(x) = self.stopped_data.take() {
            if output(x) == ValueResult::Stop {
                return GeneratorResult::Stopped;
            }
        }

        let stored_stop = &mut self.stopped_data;
        let stop_at = &mut self.stop_at;
        let result = self.data.run(|x| {
            let old_stop_at = *stop_at;
            *stop_at -= 1;
            if old_stop_at == 0 {
                *stored_stop = Some(x);
                ValueResult::Stop
            } else {
                output(x)
            }
        });
        if result == GeneratorResult::Complete {
            *stop_at = -1;
        }
        result
    }
}

/// A spuriously stopping generator that can stop multiple times.
///
/// The generator takes a slice of `Option<T>`, each `None` will result in the generator stopping,
/// otherwise a `&T` is generated.
///
/// /// ## Examples
///
/// ```
/// use pushgen::test::MultiStoppingGen;
/// use pushgen::{GeneratorResult, GeneratorExt};
/// let data = [None, Some(1), None, Some(2)];
/// let mut gen = MultiStoppingGen::new(&data);
/// assert_eq!(gen.next(), Err(GeneratorResult::Stopped));
/// assert_eq!(gen.next(), Ok(&1));
/// assert_eq!(gen.next(), Err(GeneratorResult::Stopped));
/// assert_eq!(gen.next(), Ok(&2));
/// assert_eq!(gen.next(), Err(GeneratorResult::Complete));
/// ```
pub struct MultiStoppingGen<'a, T> {
    index: usize,
    data: &'a [Option<T>],
}

impl<'a, T> MultiStoppingGen<'a, T> {
    /// Create a new multistopping generator
    pub fn new(data: &'a [Option<T>]) -> Self {
        Self {
            index: 0,
            data,
        }
    }
}

impl<'a, T> Generator for MultiStoppingGen<'a, T> {
    type Output = &'a T;

    fn run(&mut self, mut output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        while self.index < self.data.len() {
            let index = self.index;
            self.index += 1;

            match self.data[index].as_ref() {
                None => return GeneratorResult::Stopped,
                Some(value) => {
                    if output(value) == ValueResult::Stop {
                        return GeneratorResult::Stopped;
                    }
                }
            }
        }

        GeneratorResult::Complete
    }
}

#[cfg(test)]
mod tests {
    use crate::test::MultiStoppingGen;
    use crate::{GeneratorExt, GeneratorResult};

    #[test]
    fn multistop() {
        let data = [None, None, Some(1), None, Some(2), None, Some(3)];
        let mut gen = MultiStoppingGen::new(&data);
        assert_eq!(gen.next(), Err(GeneratorResult::Stopped));
        assert_eq!(gen.next(), Err(GeneratorResult::Stopped));
        assert_eq!(gen.next(), Ok(&1));
        assert_eq!(gen.next(), Err(GeneratorResult::Stopped));
        assert_eq!(gen.next(), Ok(&2));
        assert_eq!(gen.next(), Err(GeneratorResult::Stopped));
        assert_eq!(gen.next(), Ok(&3));
        assert_eq!(gen.next(), Err(GeneratorResult::Complete));
    }
}
