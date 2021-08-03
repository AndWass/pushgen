use crate::{Generator, GeneratorResult, ReverseGenerator, ValueResult};
use core::num::NonZeroUsize;

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
    // Slice can be indexed with indices in the range [begin, end). That is end is 1 greater than
    // the last index that can be used at all times.
    begin: usize,
    end: usize,
}

impl<'a, T> SliceGenerator<'a, T> {
    #[inline]
    pub fn new(slice: &'a [T]) -> Self {
        Self {
            slice,
            begin: 0,
            end: slice.len(),
        }
    }
}

impl<'a, T> Generator for SliceGenerator<'a, T> {
    type Output = &'a T;

    #[inline]
    fn run(&mut self, mut output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        let end = self.end;
        while self.begin < end {
            let index = self.begin;
            self.begin += 1;
            // Safety: index < self.end always true.
            if output(unsafe { self.slice.get_unchecked(index) }) == ValueResult::Stop {
                return GeneratorResult::Stopped;
            }
        }
        GeneratorResult::Complete
    }

    #[inline]
    fn try_advance(&mut self, n: NonZeroUsize) -> (usize, GeneratorResult) {
        let n = n.get();
        let available = self.end - self.begin;
        if n > available {
            self.begin = self.end;
            (available, GeneratorResult::Complete)
        } else {
            self.begin += n;
            (n, GeneratorResult::Stopped)
        }
    }
}

impl<'a, T> ReverseGenerator for SliceGenerator<'a, T> {
    #[inline]
    fn run_back(&mut self, mut output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        let end_back = self.begin;
        while self.end > end_back {
            // If self.end > end_back, then self.end > 0, so this will not underflow.
            let index = self.end - 1;
            // shrink the slice from the back
            self.end -= 1;
            // Safety: index always less than total len and greater or equal to 0
            if output(unsafe { self.slice.get_unchecked(index) }) == ValueResult::Stop {
                return GeneratorResult::Stopped;
            }
        }

        GeneratorResult::Complete
    }

    #[inline]
    fn try_advance_back(&mut self, n: NonZeroUsize) -> (usize, GeneratorResult) {
        let n = n.get();
        let available = self.end - self.begin;
        if n > available {
            self.end = self.begin;
            (available, GeneratorResult::Complete)
        } else {
            self.end -= n;
            (n, GeneratorResult::Stopped)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Generator, GeneratorExt};
    use core::num::NonZeroUsize;

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
        assert_eq!(result, (5, GeneratorResult::Stopped));
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

    #[test]
    fn reverse_generator() {
        let numbers = [1, 2, 3, 4, 5, 6];
        let mut gen = SliceGenerator::new(&numbers);

        assert_eq!(Ok(&1), gen.next());
        assert_eq!(Ok(&6), gen.next_back());
        assert_eq!(Ok(&5), gen.next_back());
        assert_eq!(Ok(&2), gen.next());
        assert_eq!(Ok(&3), gen.next());
        assert_eq!(Ok(&4), gen.next());
        assert_eq!(Err(GeneratorResult::Complete), gen.next());
        assert_eq!(Err(GeneratorResult::Complete), gen.next_back());
    }

    #[test]
    fn try_advance_back() {
        let data = [1, 2, 3, 4, 5];
        let mut gen = SliceGenerator::new(&data);
        let result = gen.try_advance_back(NonZeroUsize::new(3).unwrap());
        assert_eq!(result, (3, GeneratorResult::Stopped));
        assert_eq!(gen.next_back(), Ok(&2));
        assert_eq!(gen.next_back(), Ok(&1));
        assert_eq!(gen.next_back(), Err(GeneratorResult::Complete));
        assert_eq!(gen.next(), Err(GeneratorResult::Complete));
    }

    #[test]
    fn try_advance_back_inside() {
        let data = [1, 2, 3, 4, 5];
        let mut gen = SliceGenerator::new(&data);
        assert_eq!(gen.next_back(), Ok(&5));
        let result = gen.try_advance_back(NonZeroUsize::new(2).unwrap());
        assert_eq!(result, (2, GeneratorResult::Stopped));
        assert_eq!(gen.next_back(), Ok(&2));
        assert_eq!(gen.next_back(), Ok(&1));
        assert_eq!(gen.next_back(), Err(GeneratorResult::Complete));
        assert_eq!(gen.next(), Err(GeneratorResult::Complete));
    }

    #[test]
    fn try_advance_back_available() {
        let data = [1, 2, 3, 4, 5];
        let mut gen = SliceGenerator::new(&data);
        let result = gen.try_advance_back(NonZeroUsize::new(5).unwrap());
        assert_eq!(result, (5, GeneratorResult::Stopped));
        assert_eq!(gen.next_back(), Err(GeneratorResult::Complete));
        assert_eq!(gen.next(), Err(GeneratorResult::Complete));
    }

    #[test]
    fn try_advance_back_more_than_available() {
        let data = [1, 2, 3, 4, 5];
        let mut gen = SliceGenerator::new(&data);
        let result = gen.try_advance_back(NonZeroUsize::new(10).unwrap());
        assert_eq!(result, (5, GeneratorResult::Complete));
        assert_eq!(gen.next_back(), Err(GeneratorResult::Complete));
        assert_eq!(gen.next(), Err(GeneratorResult::Complete));
    }

    #[test]
    fn try_advance_front_then_back() {
        let data = [1, 2, 3, 4, 5];
        let mut gen = SliceGenerator::new(&data);

        gen.try_advance(NonZeroUsize::new(1).unwrap());
        gen.try_advance_back(NonZeroUsize::new(1).unwrap());

        assert_eq!(gen.next(), Ok(&2));
        assert_eq!(gen.next_back(), Ok(&4));
        assert_eq!(gen.next(), Ok(&3));
        assert_eq!(gen.next_back(), Err(GeneratorResult::Complete));
        assert_eq!(gen.next(), Err(GeneratorResult::Complete));
    }

    #[test]
    fn try_advance_back_then_front() {
        let data = [1, 2, 3, 4, 5];
        let mut gen = SliceGenerator::new(&data);

        gen.try_advance_back(NonZeroUsize::new(1).unwrap());
        gen.try_advance(NonZeroUsize::new(1).unwrap());

        assert_eq!(gen.next(), Ok(&2));
        assert_eq!(gen.next_back(), Ok(&4));
        assert_eq!(gen.next(), Ok(&3));
        assert_eq!(gen.next_back(), Err(GeneratorResult::Complete));
        assert_eq!(gen.next(), Err(GeneratorResult::Complete));
    }

    #[test]
    fn try_advance_back_then_front_to_end() {
        let data = [1, 2, 3, 4, 5];
        let mut gen = SliceGenerator::new(&data);

        gen.try_advance_back(NonZeroUsize::new(1).unwrap());
        assert_eq!(
            gen.try_advance(NonZeroUsize::new(5).unwrap()),
            (4, GeneratorResult::Complete)
        );

        assert_eq!(gen.next_back(), Err(GeneratorResult::Complete));
        assert_eq!(gen.next(), Err(GeneratorResult::Complete));
    }
}
