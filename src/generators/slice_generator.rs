use crate::{Generator, GeneratorResult, ValueResult};
use std::num::NonZeroUsize;

/// A generator that generates values from a slice.
///
///
/// ## Example
/// ```
/// # use pushgen::{SliceGenerator, GeneratorExt};
/// let data = [1, 2, 3, 4];
/// let mut sum = 0;
/// SliceGenerator::new(&data).for_each(|x| sum += x);
/// assert_eq!(sum, 10);
/// ```
#[derive(Clone)]
pub struct SliceGenerator<'a, T> {
    slice: &'a [T],
    index: usize,
}

impl<'a, T> SliceGenerator<'a, T> {
    #[inline]
    pub fn new(slice: &'a [T]) -> Self {
        Self { slice, index: 0 }
    }
}

impl<'a, T> Generator for SliceGenerator<'a, T> {
    type Output = &'a T;

    #[inline]
    fn run(&mut self, mut output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        // Read the len once. The Rust compiler seems to have trouble optimizing self.slice.len()
        // so read it once and use that in the loop condition instead.
        let len = self.slice.len();
        while self.index < len {
            // Safety: self.index < self.slice.len() always true.
            if output(unsafe { self.slice.get_unchecked(self.index) }) == ValueResult::Stop {
                self.index += 1;
                return GeneratorResult::Stopped;
            }
            self.index += 1;
        }
        GeneratorResult::Complete
    }

    #[inline]
    fn try_advance(&mut self, n: NonZeroUsize) -> (usize, GeneratorResult) {
        let n = n.get();
        let len = self.slice.len();
        let available = len - self.index;
        if n >= available {
            self.index = len;
            (available, GeneratorResult::Complete)
        } else {
            self.index += n;
            (n, GeneratorResult::Stopped)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Generator, GeneratorExt};
    use std::num::NonZeroUsize;

    #[test]
    fn try_advance() {
        let data = [1, 2, 3, 4, 5];
        let mut gen = SliceGenerator::new(&data);
        let result = gen.try_advance(NonZeroUsize::new(3).unwrap());
        assert_eq!(result, (3, GeneratorResult::Stopped));
        assert_eq!(gen.next(), Ok(&4));
        assert_eq!(gen.next(), Ok(&5));
    }

    #[test]
    fn try_advance_inside() {
        let data = [1, 2, 3, 4, 5];
        let mut gen = SliceGenerator::new(&data);
        assert_eq!(gen.next(), Ok(&1));
        let result = gen.try_advance(NonZeroUsize::new(2).unwrap());
        assert_eq!(result, (2, GeneratorResult::Stopped));
        assert_eq!(gen.next(), Ok(&4));
        assert_eq!(gen.next(), Ok(&5));
    }

    #[test]
    fn try_advance_available() {
        let data = [1, 2, 3, 4, 5];
        let mut gen = SliceGenerator::new(&data);
        let result = gen.try_advance(NonZeroUsize::new(5).unwrap());
        assert_eq!(result, (5, GeneratorResult::Complete));
        assert_eq!(gen.next(), Err(GeneratorResult::Complete));
    }

    #[test]
    fn try_advance_more_than_available() {
        let data = [1, 2, 3, 4, 5];
        let mut gen = SliceGenerator::new(&data);
        let result = gen.try_advance(NonZeroUsize::new(10).unwrap());
        assert_eq!(result, (5, GeneratorResult::Complete));
        assert_eq!(gen.next(), Err(GeneratorResult::Complete));
    }
}
