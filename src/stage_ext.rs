use crate::structs::{End, Filter, Then, Take, Skip, Dedup, Collect, Transform};
use crate::InputOutputStage;

pub trait StageExt: InputOutputStage {
    /// Take `amount` values and ignore any other values after that.
    ///
    /// ## Example
    /// ```
    /// # use pipe_chan::{begin, InputStage, StageExt};
    /// let mut output = Vec::new();
    /// let mut pipe = pipe_chan::begin().take(3).collect(|x| output.push(x));
    /// for x in &[1,2,3,4,5] {
    ///     pipe.process(*x);
    /// }
    /// assert_eq!(output, [1,2,3]);
    /// ```
    fn take(self, amount: usize) -> Then<Self, Take<Self::Output>>
    where
        Self: Sized
    {
        Then::new(self, Take::new(amount))
    }

    /// Skip `amount` values, then forward any remaining values after that.
    ///
    /// ## Example
    /// ```
    /// # use pipe_chan::{begin, InputStage, StageExt};
    /// let mut output = Vec::new();
    /// let mut pipe = pipe_chan::begin().skip(3).collect(|x| output.push(x));
    /// for x in &[1,2,3,4,5] {
    ///     pipe.process(*x);
    /// }
    /// assert_eq!(output, [4,5]);
    /// ```
    fn skip(self, amount: usize) -> Then<Self, Skip<Self::Output>>
        where
            Self: Sized
    {
        Then::new(self, Skip::new(amount))
    }

    /// Filter values based on a predicate.
    ///
    /// ## Example
    /// ```
    /// # use pipe_chan::{begin, InputStage, StageExt};
    /// let mut output = Vec::new();
    /// let mut pipe = pipe_chan::begin().filter(|x| x % 2 == 0).collect(|x| output.push(x));
    /// for x in 0..10 {
    ///     pipe.process(x);
    /// }
    /// assert_eq!(output, [0, 2, 4, 6, 8]);
    /// ```
    fn filter<P>(self, predicate: P) -> Then<Self, Filter<P, Self::Output>>
    where
        Self: Sized,
        P: FnMut(&Self::Output) -> bool,
    {
        Then::new(self, Filter::new(predicate))
    }

    /// Transform values using a transform function.
    ///
    /// ## Example
    /// ```
    /// # use pipe_chan::{begin, InputStage, StageExt};
    /// let mut output = Vec::new();
    /// let mut pipe = pipe_chan::begin().transform(|x| x*3).collect(|x| output.push(x));
    /// for x in 1..=3 {
    ///     pipe.process(x);
    /// }
    /// assert_eq!(output, [3,6,9]);
    /// ```
    fn transform<T, R>(self, transform: T) -> Then<Self, Transform<Self::Output, R, T>>
    where
        Self: Sized,
        T: FnMut(&Self::Output) -> R,
    {
        Then::new(self, Transform::new(transform))
    }

    /// Remove duplicates from sections of consecutive identical elements. If the input values
    /// are sorted, all elements will be unique.
    ///
    /// ## Example
    /// ```
    /// # use pipe_chan::{StageExt, InputStage};
    /// let mut output = Vec::<i32>::new();
    /// let mut pipe = pipe_chan::begin().dedup().end(|x| {
    ///     output.push(x);
    ///     true
    /// });
    ///
    /// for x in &[1,1,2,2,3,3] {
    ///     pipe.process(*x);
    /// }
    ///
    /// assert_eq!(output, [1,2,3]);
    /// ```
    fn dedup(self) -> Then<Self, Dedup<Self::Output>>
    where
        Self: Sized,
        Self::Output: PartialEq + Clone {
        Then::new(self, Dedup::new())
    }

    /// Collect all outputted values using a collector function. The collector function
    /// will always return true.
    ///
    /// ## Example
    /// ```
    /// # use pipe_chan::{StageExt, InputStage};
    /// let mut output = Vec::<i32>::new();
    /// let mut pipe = pipe_chan::begin().transform(|x| x*2).collect(|x| output.push(x));
    /// for x in &[1,2,3] {
    ///     pipe.process(*x);
    /// }
    /// assert_eq!(output, [2,4,6]);
    /// ```
    fn collect<Func>(self, collector: Func) -> Then<Self, Collect<Func, Self::Output>>
    where
        Self: Sized,
        Func: FnMut(Self::Output) {
        Then::new(self, Collect::new(collector))
    }

    fn end<T>(self, consumer: T) -> Then<Self, End<T, Self::Output>>
    where
        Self: Sized,
        T: FnMut(Self::Output) -> bool,
    {
        Then::new(self, End::new(consumer))
    }
}

impl<T: InputOutputStage> StageExt for T {}
