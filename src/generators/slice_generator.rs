use crate::{Generator, GeneratorResult, ValueResult};

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
}
