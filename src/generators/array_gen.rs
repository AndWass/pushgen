use crate::{Generator, GeneratorResult, ReverseGenerator, ValueResult};
use core::mem::MaybeUninit;
use core::num::NonZeroUsize;
use core::ops::Range;

/// A generator that generates values from an array.
///
/// Unlike [`SliceGenerator`] [`ArrayGenerator`] owns the data array, and produces `T` values instead
/// of `&T` values.
///
/// This generator can be either created manually via [`ArrayGenerator::new`] or it can be created
/// via [`IntoGenerator`] implemented for arrays.
///
/// ## Example
///
/// Basic usage:
///
/// ```
/// # use pushgen::{generators::ArrayGenerator, GeneratorExt};
/// let mut sum = 0;
/// ArrayGenerator::new([1, 2, 3, 4]).for_each(|x| sum += x);
/// assert_eq!(sum, 10);
/// ```
///
/// Creation via [`into_gen()`]:
///
/// ```
/// use pushgen::{GeneratorExt, IntoGenerator};
/// let data = [1, 2, 3, 4];
/// let copies: Vec<_> = data.into_gen().collect(); // data.into_gen() produces an ArrayGenerator
/// let refs: Vec<_> = (&data).into_gen().collect(); // (&data).into_gen() produces a SliceGenerator
/// assert_eq!(copies, [1, 2, 3, 4]);
/// assert_eq!(refs, [&1, &2, &3, &4]);
/// ```
///
/// [`SliceGenerator`]: crate::SliceGenerator
/// [`IntoGenerator`]: crate::IntoGenerator
/// [`into_gen()`]: crate::IntoGenerator::into_gen
///
pub struct ArrayGenerator<T, const N: usize> {
    data: [MaybeUninit<T>; N],
    // Slice can be indexed with indices in the range [begin, end). That is end is 1 greater than
    // the last index that can be used at all times.
    begin: usize,
    end: usize,
}

impl<T, const N: usize> ArrayGenerator<T, N> {
    /// Create a new slice generator.
    ///
    /// ## Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use pushgen::generators::ArrayGenerator;
    /// let data = [1, 2, 3];
    /// let mut gen = ArrayGenerator::new(data);
    /// ```
    #[inline]
    pub fn new(data: [T; N]) -> Self {
        Self {
            // Safety: this is a safe usage of transmute
            data: unsafe { core::mem::transmute_copy(&data) },
            begin: 0,
            end: N,
        }
    }

    // Safety requirements:
    // self.begin <= index < self.end
    #[inline]
    unsafe fn value_at(&self, index: usize) -> T {
        self.data.get_unchecked(index).as_ptr().read()
    }

    fn as_slice(&self) -> &[T] {
        // Safety: self.begin and self.end are always kept up-to-date
        unsafe {
            let slice = self.data.get_unchecked(Range {
                start: self.begin,
                end: self.end,
            });
            &*(slice as *const [MaybeUninit<T>] as *const [T])
        }
    }

    fn uninit_data_array() -> [MaybeUninit<T>; N] {
        // Safety: This is the exact implementation of MaybeUninit::uninit_array()
        unsafe { MaybeUninit::<[MaybeUninit<T>; N]>::uninit().assume_init() }
    }
}

impl<T: Clone, const N: usize> Clone for ArrayGenerator<T, N> {
    #[inline]
    fn clone(&self) -> Self {
        let mut new = Self {
            begin: 0,
            end: 0,
            data: Self::uninit_data_array(),
        };

        for (o, n) in self.as_slice().iter().zip(new.data.iter_mut()) {
            // Safety: n is always valid
            unsafe { n.as_mut_ptr().write(o.clone()) };
            new.end += 1;
        }

        new
    }
}

impl<T, const N: usize> Generator for ArrayGenerator<T, N> {
    type Output = T;

    #[inline]
    fn run(&mut self, mut output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        let end = self.end;
        while self.begin < end {
            // Safety: self.begin < self.end always true.
            if output(unsafe { self.value_at(self.begin) }) == ValueResult::Stop {
                self.begin += 1;
                return GeneratorResult::Stopped;
            }
            self.begin += 1;
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

impl<T, const N: usize> ReverseGenerator for ArrayGenerator<T, N> {
    #[inline]
    fn run_back(&mut self, mut output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        let end_back = self.begin;
        while self.end > end_back {
            // self.end > end_back -> self.end > 0, so self.end-1 is safe
            // Safety: self.end-1 always in range [0, self.slice.len())
            if output(unsafe { self.value_at(self.end - 1) }) == ValueResult::Stop {
                self.end -= 1;
                return GeneratorResult::Stopped;
            }
            self.end -= 1;
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

impl<T, const N: usize> Drop for ArrayGenerator<T, N> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            // Safety: This is ported from what is done by std lib but available in a stable API yet
            // See core::iter::IntoIter<T, N> drop code.
            // Safety: begin and end always inside the original slice
            let slice = self.data.get_unchecked_mut(Range {
                start: self.begin,
                end: self.end,
            });
            let slice = &mut *(slice as *mut [MaybeUninit<T>] as *mut [T]);
            core::ptr::drop_in_place(slice);
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Generator, GeneratorExt};
    use core::num::NonZeroUsize;
    use std::sync::atomic::{AtomicU32, Ordering};

    #[test]
    fn try_advance() {
        let data = [1, 2, 3, 4, 5];
        let mut gen = ArrayGenerator::new(data);
        let result = gen.try_advance(NonZeroUsize::new(3).unwrap());
        assert_eq!(result, (3, GeneratorResult::Stopped));
        assert_eq!(gen.next(), Ok(4));
        assert_eq!(gen.next(), Ok(5));
    }

    #[test]
    fn try_advance_inside() {
        let data = [1, 2, 3, 4, 5];
        let mut gen = ArrayGenerator::new(data);
        assert_eq!(gen.next(), Ok(1));
        let result = gen.try_advance(NonZeroUsize::new(2).unwrap());
        assert_eq!(result, (2, GeneratorResult::Stopped));
        assert_eq!(gen.next(), Ok(4));
        assert_eq!(gen.next(), Ok(5));
    }

    #[test]
    fn try_advance_available() {
        let data = [1, 2, 3, 4, 5];
        let mut gen = ArrayGenerator::new(data);
        let result = gen.try_advance(NonZeroUsize::new(5).unwrap());
        assert_eq!(result, (5, GeneratorResult::Stopped));
        assert_eq!(gen.next(), Err(GeneratorResult::Complete));
    }

    #[test]
    fn try_advance_more_than_available() {
        let data = [1, 2, 3, 4, 5];
        let mut gen = ArrayGenerator::new(data);
        let result = gen.try_advance(NonZeroUsize::new(10).unwrap());
        assert_eq!(result, (5, GeneratorResult::Complete));
        assert_eq!(gen.next(), Err(GeneratorResult::Complete));
    }

    #[test]
    fn reverse_generator() {
        let numbers = [1, 2, 3, 4, 5, 6];
        let mut gen = ArrayGenerator::new(numbers);

        assert_eq!(Ok(1), gen.next());
        assert_eq!(Ok(6), gen.next_back());
        assert_eq!(Ok(5), gen.next_back());
        assert_eq!(Ok(2), gen.next());
        assert_eq!(Ok(3), gen.next());
        assert_eq!(Ok(4), gen.next());
        assert_eq!(Err(GeneratorResult::Complete), gen.next());
        assert_eq!(Err(GeneratorResult::Complete), gen.next_back());
    }

    #[test]
    fn try_advance_back() {
        let data = [1, 2, 3, 4, 5];
        let mut gen = ArrayGenerator::new(data);
        let result = gen.try_advance_back(NonZeroUsize::new(3).unwrap());
        assert_eq!(result, (3, GeneratorResult::Stopped));
        assert_eq!(gen.next_back(), Ok(2));
        assert_eq!(gen.next_back(), Ok(1));
        assert_eq!(gen.next_back(), Err(GeneratorResult::Complete));
        assert_eq!(gen.next(), Err(GeneratorResult::Complete));
    }

    #[test]
    fn try_advance_back_inside() {
        let data = [1, 2, 3, 4, 5];
        let mut gen = ArrayGenerator::new(data);
        assert_eq!(gen.next_back(), Ok(5));
        let result = gen.try_advance_back(NonZeroUsize::new(2).unwrap());
        assert_eq!(result, (2, GeneratorResult::Stopped));
        assert_eq!(gen.next_back(), Ok(2));
        assert_eq!(gen.next_back(), Ok(1));
        assert_eq!(gen.next_back(), Err(GeneratorResult::Complete));
        assert_eq!(gen.next(), Err(GeneratorResult::Complete));
    }

    #[test]
    fn try_advance_back_available() {
        let data = [1, 2, 3, 4, 5];
        let mut gen = ArrayGenerator::new(data);
        let result = gen.try_advance_back(NonZeroUsize::new(5).unwrap());
        assert_eq!(result, (5, GeneratorResult::Stopped));
        assert_eq!(gen.next_back(), Err(GeneratorResult::Complete));
        assert_eq!(gen.next(), Err(GeneratorResult::Complete));
    }

    #[test]
    fn try_advance_back_more_than_available() {
        let data = [1, 2, 3, 4, 5];
        let mut gen = ArrayGenerator::new(data);
        let result = gen.try_advance_back(NonZeroUsize::new(10).unwrap());
        assert_eq!(result, (5, GeneratorResult::Complete));
        assert_eq!(gen.next_back(), Err(GeneratorResult::Complete));
        assert_eq!(gen.next(), Err(GeneratorResult::Complete));
    }

    #[test]
    fn try_advance_front_then_back() {
        let data = [1, 2, 3, 4, 5];
        let mut gen = ArrayGenerator::new(data);

        gen.try_advance(NonZeroUsize::new(1).unwrap());
        gen.try_advance_back(NonZeroUsize::new(1).unwrap());

        assert_eq!(gen.next(), Ok(2));
        assert_eq!(gen.next_back(), Ok(4));
        assert_eq!(gen.next(), Ok(3));
        assert_eq!(gen.next_back(), Err(GeneratorResult::Complete));
        assert_eq!(gen.next(), Err(GeneratorResult::Complete));
    }

    #[test]
    fn try_advance_back_then_front() {
        let data = [1, 2, 3, 4, 5];
        let mut gen = ArrayGenerator::new(data);

        gen.try_advance_back(NonZeroUsize::new(1).unwrap());
        gen.try_advance(NonZeroUsize::new(1).unwrap());

        assert_eq!(gen.next(), Ok(2));
        assert_eq!(gen.next_back(), Ok(4));
        assert_eq!(gen.next(), Ok(3));
        assert_eq!(gen.next_back(), Err(GeneratorResult::Complete));
        assert_eq!(gen.next(), Err(GeneratorResult::Complete));
    }

    #[test]
    fn try_advance_back_then_front_to_end() {
        let data = [1, 2, 3, 4, 5];
        let mut gen = ArrayGenerator::new(data);

        gen.try_advance_back(NonZeroUsize::new(1).unwrap());
        assert_eq!(
            gen.try_advance(NonZeroUsize::new(5).unwrap()),
            (4, GeneratorResult::Complete)
        );

        assert_eq!(gen.next_back(), Err(GeneratorResult::Complete));
        assert_eq!(gen.next(), Err(GeneratorResult::Complete));
    }

    #[test]
    fn dropping() {
        static COUNTER: AtomicU32 = AtomicU32::new(0);

        #[derive(Eq, Ord, PartialOrd, PartialEq, Debug)]
        struct Tracked(u32);

        impl Drop for Tracked {
            fn drop(&mut self) {
                COUNTER.fetch_add(1, Ordering::AcqRel);
            }
        }

        let mut gen = ArrayGenerator::new([Tracked(0), Tracked(1), Tracked(2), Tracked(3)]);
        assert_eq!(gen.next(), Ok(Tracked(0)));
        assert_eq!(gen.next_back(), Ok(Tracked(3)));

        COUNTER.store(0, Ordering::Release);
        drop(gen);
        assert_eq!(COUNTER.load(Ordering::Acquire), 2);
    }
}
