use crate::structs::{
    Chain, Cloned, Copied, Dedup, Filter, FilterMap, Flatten, IteratorAdaptor, Map, Skip,
    SkipWhile, Take, TakeWhile, Zip,
};
use crate::traits::{Product, Sum};
use crate::{Generator, GeneratorResult, ValueResult};
use core::cmp::Ordering;

pub trait Sealed {}

impl<T> Sealed for T where T: Generator {}

/// Provides extension-methods for all generators.
///
/// This allows generators to be composed to new generators, or consumed.
///
/// ## Example
/// ```
/// use pushgen::{SliceGenerator, GeneratorExt};
/// let data = [1, 2, 3, 4];
/// let mut output: Vec<i32> = Vec::new();
/// SliceGenerator::new(&data).map(|x| x*3).for_each(|x| output.push(x));
/// assert_eq!(output, [3,6,9,12]);
/// ```
pub trait GeneratorExt: Sealed + Generator {
    /// Tests if every value from the generator matches a predicate.
    ///
    /// `all()` takes a closure that returns `true` or `false`. It applies this closure to each
    /// value generated by the generator, and if they all return `true`, then so does `all()`. If
    /// any  value returns `false`, `all()` returns `false`.
    ///
    /// `all()` is short-circuiting; it will stop processing as soon as it finds a `false`.
    ///
    /// An empty generator returns true.
    ///
    /// ## Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use pushgen::{IntoGenerator, GeneratorExt};
    /// let a = [1, 2, 3];
    /// assert!(a.into_gen().all(|&x| x > 0));
    /// assert!(!a.into_gen().all(|&x| x > 2));
    /// ```
    ///
    /// Stopping at first false:
    ///
    /// ```
    /// use pushgen::{IntoGenerator, GeneratorExt};
    /// let a = [1, 2, 3];
    /// let mut gen = a.into_gen();
    /// assert!(!gen.all(|&x| x != 2));
    /// assert_eq!(gen.iter().next(), Some(&3));
    /// ```
    #[inline]
    fn all<F>(&mut self, mut predicate: F) -> bool
    where
        F: FnMut(Self::Output) -> bool,
    {
        let mut retval = true;
        self.run(|x| {
            if !predicate(x) {
                retval = false;
                ValueResult::Stop
            } else {
                ValueResult::MoreValues
            }
        });
        retval
    }

    /// Tests if any value matches a predicate.
    ///
    /// `any()` takes a closure that returns `true` or `false`. It applies
    /// this closure to each value from the generator, and if any of them return
    /// `true`, then so does `any()`. If they all return `false`, it
    /// returns `false`.
    ///
    /// `any()` is short-circuiting; in other words, it will stop processing
    /// as soon as it finds a `true`, given that no matter what else happens,
    /// the result will also be `true`.
    ///
    /// An empty generator returns `false`.
    ///
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use pushgen::{IntoGenerator, GeneratorExt};
    /// let a = [1, 2, 3];
    ///
    /// assert!(a.into_gen().any(|&x| x > 0));
    ///
    /// assert!(!a.into_gen().any(|&x| x > 5));
    /// ```
    ///
    /// Stopping at the first `true`:
    ///
    /// ```
    /// use pushgen::{IntoGenerator, GeneratorExt};
    /// let a = [1, 2, 3];
    ///
    /// let mut gen = a.into_gen();
    ///
    /// assert!(gen.any(|&x| x != 2));
    ///
    /// // we can still use `gen`, as there are more elements.
    /// assert_eq!(gen.iter().next(), Some(&2));
    /// ```
    #[inline]
    fn any<F>(&mut self, mut predicate: F) -> bool
    where
        F: FnMut(Self::Output) -> bool,
    {
        let mut retval = false;
        self.run(|x| {
            if predicate(x) {
                retval = true;
                ValueResult::Stop
            } else {
                ValueResult::MoreValues
            }
        });
        retval
    }
    /// Exhausts the generator, returning the last element.
    ///
    /// This method will evaluate the generator until it completes. While
    /// doing so, it keeps track of the current element. After it completes
    /// `last()` will then return the last element it saw.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use pushgen::IntoGenerator;
    /// use pushgen::GeneratorExt;
    ///
    /// let a = [1, 2, 3];
    /// assert_eq!(a.into_gen().last(), Some(&3));
    ///
    /// let a = [1, 2, 3, 4, 5];
    /// assert_eq!(a.into_gen().last(), Some(&5));
    /// ```
    #[inline]
    fn last(mut self) -> Option<Self::Output>
    where
        Self: Sized,
    {
        let mut res = None;
        let res_mut = &mut res;
        self.run(move |value| {
            *res_mut = Some(value);
            ValueResult::MoreValues
        });
        res
    }

    /// Creates a generator that clones all of its elements.
    ///
    /// This is useful when you have a generator that generates `&T` but you need a generate
    /// that generates `T`.
    ///
    /// ## Examples
    ///
    /// Basic usage
    ///
    /// ```rust
    /// use pushgen::{SliceGenerator, GeneratorExt};
    /// let data = [1, 2, 3];
    /// let mut output: Vec<i32> = Vec::new();
    /// SliceGenerator::new(&data).cloned().for_each(|x| output.push(x));
    /// assert_eq!(output, [1, 2, 3])
    /// ```
    #[inline]
    fn cloned<'a, T>(self) -> Cloned<Self>
    where
        Self: Generator<Output = &'a T> + Sized,
        T: 'a + Clone,
    {
        Cloned::new(self)
    }

    /// Creates a generator that copies all of its elements.
    ///
    /// This is useful when you have a generator of `&T` but need a generator of `T`.
    ///
    /// ## Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// use pushgen::{SliceGenerator, GeneratorExt};
    /// let data = [1, 2, 3];
    /// let mut v_copied: Vec<_> = Vec::new();
    /// SliceGenerator::new(&data).copied().for_each(|x| v_copied.push(x));
    ///
    /// assert_eq!(v_copied, [1, 2, 3]);
    /// ```
    #[inline]
    fn copied<'a, T>(self) -> Copied<Self>
    where
        T: 'a + Copy,
        Self: Generator<Output = &'a T> + Sized,
    {
        Copied::new(self)
    }

    /// Creates a generator by chaining two generators, running them one after the other.
    ///
    /// ## Example
    /// ```
    /// use pushgen::{SliceGenerator, GeneratorExt};
    /// let data = [1, 2, 3];
    /// let mut output: Vec<i32> = Vec::new();
    /// SliceGenerator::new(&data).chain(SliceGenerator::new(&data)).for_each(|x| output.push(*x));
    /// assert_eq!(output, [1, 2, 3, 1, 2, 3]);
    /// ```
    #[inline]
    fn chain<Gen>(self, other: Gen) -> Chain<Self, Gen>
    where
        Self: Sized,
        Gen: Generator<Output = Self::Output>,
    {
        Chain::new(self, other)
    }

    /// Create a filtered generator. Only values for which the predicate returns true will be passed on.
    ///
    /// The predicate must implement `FnMut(&Gen::Output) -> bool`.
    ///
    /// ## Example
    /// ```
    /// # use pushgen::*;
    /// let input = [1,2,3,4];
    /// let mut output: Vec<i32> = Vec::new();
    /// let run_result = SliceGenerator::new(&input).filter(|x| *x % 2 == 0).for_each(|x| output.push(*x));
    /// assert_eq!(run_result, GeneratorResult::Complete);
    /// assert_eq!(output, [2,4]);
    /// ```
    #[inline]
    fn filter<Pred>(self, predicate: Pred) -> Filter<Self, Pred>
    where
        Self: Sized,
        Pred: FnMut(&Self::Output) -> bool,
    {
        Filter::new(self, predicate)
    }

    /// Creates a generator that both filters and maps.
    ///
    /// The returned generator produces only the `value`s for which the supplied
    /// closure returns `Some(value)`.
    ///
    /// `filter_map` can be used to make chains of [`filter`] and [`map`] more
    /// concise. The example below shows how a `map().filter().map()` can be
    /// shortened to a single call to `filter_map`.
    ///
    /// [`filter`]: GeneratorExt::filter
    /// [`map`]: GeneratorExt::map
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use pushgen::{SliceGenerator, GeneratorExt};
    ///
    /// let a = ["1", "two", "NaN", "four", "5"];
    ///
    /// let mut output: Vec<i32> = Vec::new();
    /// SliceGenerator::new(&a).filter_map(|s| s.parse().ok()).for_each(|x: i32| output.push(x));
    /// assert_eq!(output, [1, 5]);
    /// ```
    ///
    /// Here's the same example, but with [`filter`] and [`map`]:
    ///
    /// ```
    /// use pushgen::{SliceGenerator, GeneratorExt};
    ///
    /// let a = ["1", "two", "NaN", "four", "5"];
    ///
    /// let mut output: Vec<i32> = Vec::new();
    /// SliceGenerator::new(&a).map(|s| s.parse()).filter(|s| s.is_ok()).map(|s| s.unwrap()).for_each(|x: i32| output.push(x));
    /// assert_eq!(output, [1, 5]);
    /// ```
    #[inline]
    fn filter_map<B, F>(self, f: F) -> FilterMap<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Output) -> Option<B>,
    {
        FilterMap::new(self, f)
    }

    /// Takes a closure and creates a generator which  calls the closure on each value.
    ///
    /// ## Example
    /// ```
    /// use pushgen::{SliceGenerator, GeneratorExt};
    /// let data = [1, 2, 3];
    /// let mut output: Vec<String> = Vec::new();
    /// SliceGenerator::new(&data).map(|x| x.to_string()).for_each(|x| output.push(x));
    /// assert_eq!(output, ["1", "2", "3"]);
    /// ```
    #[inline]
    fn map<Trans, Out>(self, transform_fn: Trans) -> Map<Self, Trans>
    where
        Self: Sized,
        Trans: FnMut(Self::Output) -> Out,
    {
        Map::new(self, transform_fn)
    }

    /// Skips over `n` values, consuming and ignoring them.
    ///
    /// ## Example
    ///```
    /// # use pushgen::{GeneratorExt, SliceGenerator};
    /// # use pushgen::structs::Skip;
    /// let input = [1,2,3,4];
    /// let mut skipped_generator = SliceGenerator::new(&input).skip(2);
    /// let mut output: Vec<i32> = Vec::new();
    /// skipped_generator.for_each(|x| output.push(*x));
    /// assert_eq!(output, [3,4]);
    /// ```
    #[inline]
    fn skip(self, n: usize) -> Skip<Self>
    where
        Self: Sized,
    {
        Skip::new(self, n)
    }

    /// Creates a generator that skips values based on a predicate.
    ///
    /// `skip_while()` takes a closure as argument. It will call this closure on each value,
    /// and ignore values until the closure returns `false`.
    ///
    /// After `false` is returned, `skip_while()` will push the rest of the values.
    ///
    /// ## Examples
    ///
    /// Basic usage
    ///
    /// ```rust
    /// use pushgen::{IntoGenerator, GeneratorExt};
    /// let a = [-1i32, 0, 1];
    /// let mut output = Vec::new();
    /// a.into_gen().skip_while(|x| x.is_negative()).for_each(|x| output.push(x));
    /// assert_eq!(output, [&0, &1]);
    /// ```
    #[inline]
    fn skip_while<P>(self, predicate: P) -> SkipWhile<Self, P>
    where
        Self: Sized,
        P: FnMut(&Self::Output) -> bool,
    {
        SkipWhile::new(self, predicate)
    }

    /// Takes `n` values and then completes the generator.
    ///
    /// ## Example
    /// ```
    /// use pushgen::{SliceGenerator, GeneratorExt};
    /// let data = [1, 2, 3, 4];
    /// let mut output: Vec<i32> = Vec::new();
    /// SliceGenerator::new(&data).take(2).for_each(|x| output.push(*x));
    /// assert_eq!(output, [1, 2]);
    /// ```
    #[inline]
    fn take(self, n: usize) -> Take<Self>
    where
        Self: Sized,
    {
        Take::new(self, n)
    }

    /// Creates a generator that pushes values based on a predicate.
    ///
    /// `take_while()` takes a closure as an argument. It will call this closure on each value
    /// received from the source generator, and push values while it returns true. After `false` is
    /// returned, `take_while()`'s job is over and it will always report `Complete`.
    ///
    /// ## Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// use pushgen::{IntoGenerator, GeneratorExt};
    /// let a = [-1i32, 0, 1];
    ///
    /// let mut gen_as_iter = a.into_gen().take_while(|x| x.is_negative()).iter();
    ///
    /// assert_eq!(gen_as_iter.next(), Some(&-1));
    /// assert_eq!(gen_as_iter.next(), None);
    /// ```
    #[inline]
    fn take_while<P>(self, predicate: P) -> TakeWhile<Self, P>
    where
        Self: Sized,
        P: FnMut(&Self::Output) -> bool,
    {
        TakeWhile::new(self, predicate)
    }

    /// Creates a generator that works like map, but flattens nested structure.
    ///
    /// The [`map`] adapter is very useful, but only when the closure
    /// argument produces values. If it produces a generator instead, there's
    /// an extra layer of indirection. `flat_map()` will remove this extra layer
    /// on its own.
    ///
    /// You can think of `flat_map(f)` as the semantic equivalent
    /// of [`map`]ping, and then [`flatten`]ing as in `map(f).flatten()`.
    ///
    /// Another way of thinking about `flat_map()`: [`map`]'s closure returns
    /// one item for each element, and `flat_map()`'s closure returns an
    /// iterator for each element.
    ///
    /// [`map`]: GeneratorExt::map
    /// [`flatten`]: GeneratorExt::flatten
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use pushgen::IntoGenerator;
    /// use crate::pushgen::GeneratorExt;
    ///
    /// let words = ["alpha", "beta", "gamma"];
    ///
    /// let mut merged = String::new();
    /// words.into_gen()
    ///      .flat_map(|s| pushgen::from_iter(s.chars()))
    ///      .for_each(|x| merged.push(x));
    /// assert_eq!(merged, "alphabetagamma");
    /// ```
    #[inline]
    fn flat_map<U, F>(self, f: F) -> Flatten<Map<Self, F>>
    where
        Self: Sized,
        U: crate::IntoGenerator,
        F: FnMut(Self::Output) -> U,
    {
        self.map(f).flatten()
    }

    /// Creates a generator that flattens nested structure.
    ///
    /// This is useful when you have a generator of generators or a generator of
    /// things that can be turned into generators and you want to remove one
    /// level of indirection.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use pushgen::IntoGenerator;
    /// use crate::pushgen::GeneratorExt;
    ///
    /// let data = vec![vec![1, 2, 3, 4], vec![5, 6]];
    /// let mut output: Vec<i32> = Vec::new();
    /// let flattened = data.into_gen().flatten().for_each(|x| output.push(x));
    /// assert_eq!(output, [1, 2, 3, 4, 5, 6]);
    /// ```
    ///
    /// Mapping and then flattening:
    ///
    /// ```
    /// use pushgen::IntoGenerator;
    /// use crate::pushgen::GeneratorExt;
    ///
    /// let words = &["alpha", "beta", "gamma"];
    ///
    /// let mut merged = String::new();
    /// words.into_gen()
    ///      .map(|s| pushgen::from_iter(s.chars()))
    ///      .flatten()
    ///      .for_each(|x| merged.push(x));
    /// assert_eq!(merged, "alphabetagamma");
    /// ```
    #[inline]
    fn flatten(self) -> Flatten<Self>
    where
        Self: Sized,
        Self::Output: crate::IntoGenerator,
    {
        Flatten::new(self)
    }

    /// Run a generator to completion, or until it is stopped, and call a closure for each value
    /// produced by the generator.
    ///
    /// The closure will be called for as long as the generator produces values, it is not possible
    /// to abort processing early. If early abort is needed, use [`Generator::run`](crate::Generator::run)
    /// ## Example
    /// ```
    /// # use pushgen::{GeneratorExt, GeneratorResult, SliceGenerator};
    /// let mut sum = 0i32;
    /// let data = [1,2,3];
    /// let result = SliceGenerator::new(&data).for_each(|x| sum += x);
    /// assert_eq!(sum, 6);
    /// assert_eq!(result, GeneratorResult::Complete);
    /// ```
    #[inline]
    fn for_each<Func>(&mut self, mut func: Func) -> GeneratorResult
    where
        Self: Sized,
        Func: FnMut(Self::Output),
    {
        self.run(move |value| {
            func(value);
            ValueResult::MoreValues
        })
    }

    /// A generator method that applies a fallible function to each item
    /// produced, stopping at the first error and returning that error.
    ///
    /// This can also be thought of as the fallible form of [`for_each()`]
    /// or as the stateless version of [`try_fold()`].
    ///
    /// [`for_each()`]: Generator::for_each
    /// [`try_fold()`]: Generator::try_fold
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fs::rename;
    /// use std::io::{stdout, Write};
    /// use std::path::Path;
    /// use pushgen::{SliceGenerator, GeneratorExt};
    ///
    /// let data = ["no_tea.txt", "stale_bread.json", "torrential_rain.png"];
    ///
    /// let res = SliceGenerator::new(&data).try_for_each(|x| writeln!(stdout(), "{}", x));
    /// assert!(res.is_ok());
    ///
    /// let mut gen = SliceGenerator::new(&data);
    /// let res = gen.try_for_each(|x| rename(x, Path::new(x).with_extension("old")));
    /// assert!(res.is_err());
    /// // It short-circuited, so the remaining items are still in the iterator:
    /// let mut output: Vec<&'static str> = Vec::new();
    /// gen.for_each(|x| output.push(*x));
    /// assert_eq!(output, ["stale_bread.json", "torrential_rain.png"]);
    /// ```
    #[inline]
    fn try_for_each<F, E>(&mut self, mut f: F) -> Result<(), E>
    where
        Self: Sized,
        F: FnMut(Self::Output) -> Result<(), E>,
    {
        let mut res = Ok(());
        let res_mut = &mut res;
        self.run(move |value| match f(value) {
            Ok(()) => ValueResult::MoreValues,
            Err(e) => {
                *res_mut = Err(e);
                ValueResult::Stop
            }
        });
        res
    }

    /// Zips the output of two generators into a single generator of pairs.
    ///
    /// `zip()` returns a new generator that will use values from two generators, outputting
    /// a tuple where the first element comes from the first generator, and the second element comes
    /// from the second generator.
    ///
    /// The zip generator will complete when either generator completes.
    ///
    /// ## Example
    /// ```
    /// use pushgen::{SliceGenerator, GeneratorExt};
    /// let left = [1, 2, 3];
    /// let right = [4, 5, 6];
    /// let mut output: Vec<(i32, i32)> = Vec::new();
    /// SliceGenerator::new(&left).zip(SliceGenerator::new(&right)).for_each(|(a, b)| output.push((*a, *b)));
    /// assert_eq!(output, [(1,4), (2, 5), (3, 6)]);
    /// ```
    #[inline]
    fn zip<Right>(self, right: Right) -> Zip<Self, Right>
    where
        Self: Sized,
        Right: Generator,
    {
        Zip::new(self, right)
    }

    /// Create a de-duplicating generator, removing consecutive duplicate values.
    ///
    /// Values will be made available when a non-duplicate is detected. If the up-stream generator generates
    /// the following sequence: `[1, 2, 3, 3, 4]` then the value `1` will be generated from the
    /// `Dedup` generator once the value `2` has been generated by the upstream generator and so
    /// on.
    ///
    /// | Upstream value | Dedup-generated value |
    /// |----------------|-----------------------|
    /// | 1              | *None*                |
    /// | 2              | 1                     |
    /// | 3              | 2                     |
    /// | 3              | *Ignored*             |
    /// | 4              | 3                     |
    /// | *Complete*     | 4                     |
    /// | *Complete*     | *Complete*            |
    ///
    /// ## Example
    /// ```
    /// # use pushgen::{SliceGenerator, GeneratorExt};
    /// let data = [1, 2, 3, 3, 3, 3, 4, 3];
    /// let mut output: Vec<i32> = Vec::new();
    /// SliceGenerator::new(&data).dedup().for_each(|x| output.push(*x));
    /// assert_eq!(output, [1, 2, 3, 4, 3]);
    /// ```
    #[inline]
    fn dedup(self) -> Dedup<Self>
    where
        Self: Sized,
        Self::Output: PartialEq,
    {
        Dedup::new(self)
    }

    /// Create an iterator from a generator.
    ///
    /// This allows generators to be used in basic for-loops.
    ///
    /// ## Example
    /// ```
    /// use pushgen::{SliceGenerator, GeneratorExt};
    /// let data = [1, 2, 3, 4, 5, 6];
    /// let mut sum = 0;
    /// for x in SliceGenerator::new(&data).iter() {
    ///     sum += x;
    /// }
    /// assert_eq!(sum, data.iter().sum());
    /// ```
    #[inline]
    fn iter(self) -> IteratorAdaptor<Self>
    where
        Self: Sized,
    {
        IteratorAdaptor::new(self)
    }

    /// Box a generator, making it possible to use as return value in for instance traits.
    ///
    /// ## Performance
    /// This causes at least one layer of redirection, which is very likely to impact performance.
    /// One should always prefer to use `impl Generator<Output=X>` instead.
    ///
    /// ## Example
    /// ```rust
    /// use pushgen::{BoxedGenerator, IntoGenerator, GeneratorExt};
    /// fn make_generator() -> BoxedGenerator<i32> {
    ///     vec![1, 2, 3, 4].into_gen().map(|x| x*2).boxed()
    /// }
    /// let mut output = Vec::new();
    /// make_generator().for_each(|x| output.push(x));
    /// assert_eq!(output, [2, 4, 6, 8]);
    /// ```
    #[cfg(feature = "std")]
    #[inline]
    fn boxed(self) -> crate::BoxedGenerator<Self::Output>
    where
        Self: Sized + 'static,
    {
        crate::BoxedGenerator::new(self)
    }

    /// Sums the values of a generator. Takes each value and adds them together and returns
    /// the result.
    ///
    /// An empty generator returns the zero value of the type.
    ///
    /// ## Spuriously stopping generators
    ///
    /// `sum()` only sums the values up until the source generator is first stopped. If the source
    /// generator is not completed, but stops mid-generation for some reason, only the values up
    /// until the first stop are summed.
    ///
    /// ## Panics
    ///
    /// When calling `sum()` and a primitive integer type is being returned,
    /// this method will panic if the computation overflows and debug assertions are enabled.
    ///
    /// ## Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use pushgen::{IntoGenerator, GeneratorExt};
    /// let a = [1, 2, 3];
    /// let sum: i32 = a.into_gen().sum();
    ///
    /// assert_eq!(sum, 6);
    /// ```
    ///
    #[inline]
    fn sum<S>(self) -> S
    where
        Self: Sized,
        S: Sum<Self::Output>,
    {
        S::sum(self)
    }

    /// Multiplies the values of a generator. Takes each value and adds them together and returns
    /// the result.
    ///
    /// An empty generator returns the one value of the type.
    ///
    /// ## Spuriously stopping generators
    ///
    /// `product()` only multiplies the values up until the source generator is first stopped. If the source
    /// generator is not completed, but stops mid-generation for some reason, only the values up
    /// until the first stop are multiplied.
    ///
    /// ## Panics
    ///
    /// When calling `product()` and a primitive integer type is being returned,
    /// this method will panic if the computation overflows and debug assertions are enabled.
    ///
    /// ## Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use pushgen::{GeneratorExt, from_iter};
    /// fn factorial(n: u32) -> u32 {
    ///     // Create a generator from an iterable
    ///     from_iter((1..=n)).product()
    /// }
    ///
    /// assert_eq!(factorial(0), 1);
    /// assert_eq!(factorial(1), 1);
    /// assert_eq!(factorial(5), 120);
    /// ```
    ///
    #[inline]
    fn product<P>(self) -> P
    where
        Self: Sized,
        P: Product<Self::Output>,
    {
        P::product(self)
    }

    /// Returns the value that gives the minimum value when compared with the
    /// specified comparison function.
    ///
    /// If several elements are equally minimum, the first element is
    /// returned. If the iterator is empty, [`None`] is returned.
    ///
    /// ## Spuriously stopping generators
    ///
    /// `min_by()` will return the result after the source generator has stopped. It doesn't matter
    /// if the source generator is stopped or completed.
    ///
    /// Manually use [`try_reduce`] to handle spuriously stopping generators.
    ///
    /// [`try_reduce`]: GeneratorExt::try_reduce
    ///
    /// # Examples
    ///
    /// ```
    /// use pushgen::{GeneratorExt, IntoGenerator};
    /// let a = [-3_i32, 0, 1, 5, -10];
    /// assert_eq!(*a.into_gen().min_by(|x, y| x.cmp(y)).unwrap(), -10);
    /// ```
    #[inline]
    fn min_by<F>(self, mut compare: F) -> Option<Self::Output>
    where
        Self: Sized,
        F: FnMut(&Self::Output, &Self::Output) -> Ordering {
        self.reduce(|a, b| core::cmp::min_by(a, b, &mut compare))
    }

    /// Reduces the elements to a single one by repeatedly applying a reducing operation.
    ///
    /// ## Returns
    ///
    /// `None` if the generator is empty, otherwise the result of the reduction.
    ///
    /// ## Spuriously stopping generators
    ///
    /// Reduce will return the result after the source generator has stopped. It doesn't matter
    /// if the source generator is stopped or completed.
    ///
    /// Use [`try_reduce`] to reduce spuriously stopping generators.
    ///
    /// [`try_reduce`]: GeneratorExt::try_reduce
    ///
    /// ## Example
    ///
    /// Find the maximum value:
    ///
    /// ```
    /// use pushgen::{Generator, GeneratorExt, IntoGenerator};
    /// fn find_max<G>(gen: G) -> Option<G::Output>
    ///     where G: Generator,
    ///           G::Output: Ord,
    /// {
    ///     gen.reduce(|a, b| {
    ///         if a >= b { a } else { b }
    ///     })
    /// }
    /// let a = [10, 20, 5, -23, 0];
    /// let b: [u32; 0] = [];
    ///
    /// assert_eq!(find_max(a.into_gen()), Some(&20));
    /// assert_eq!(find_max(b.into_gen()), None);
    /// ```
    ///
    #[inline]
    fn reduce<F>(mut self, mut reducer: F) -> Option<Self::Output>
    where
        Self: Sized,
        F: FnMut(Self::Output, Self::Output) -> Self::Output,
    {
        let mut left_value = {
            // Grab the first item into an optional
            let mut first = None;
            self.run(|x| {
                first = Some(x);
                ValueResult::Stop
            });

            // In the hot loop we use an inplace updatable since we know we will never
            // have a None option from now on.
            crate::structs::utility::InplaceUpdatable::new(first?)
        };

        self.run(|x| {
            left_value.inplace_reduce(x, &mut reducer);
            ValueResult::MoreValues
        });

        Some(left_value.get_inner())
    }

    /// Reduces the values to a single value by repeatedly applying a reducing operation.
    ///
    /// Use this reduction if the generator is known to spuriously stop mid-stream. Otherwise
    /// it is better to use [`reduce()`].
    ///
    /// [`reduce()`]: GeneratorExt::reduce
    ///
    /// ## Arguments
    ///
    /// `prev_reduction` The result of an earlier incomplete reduction. Set to `None` if this is the
    /// first reduction pass.
    ///
    /// `reducer` The reducing  closure to use.
    ///
    /// ## Returns
    ///
    /// `Ok(x)` if the generator was run to completion. `x` is `None` if the generator is empty,
    /// otherwise it is the result of the complete reduction.
    ///
    /// `Err(y)` if the generator was stopped mid-reduction. `y` is the value that the generator was
    /// reduced to when it stopped. This value should be used in any subsequent calls to `try_reduce`
    /// until an `Ok()` value is returned.
    ///
    /// ## Example
    ///
    /// Find the maximum value:
    ///
    /// ```
    /// use pushgen::{Generator, GeneratorExt, IntoGenerator};
    /// fn find_max<G>(gen: &mut G) -> Result<Option<G::Output>, Option<G::Output>>
    ///     where G: Generator,
    ///           G::Output: Ord,
    /// {
    ///     gen.try_reduce(None, |a, b| {
    ///         if a >= b { a } else { b }
    ///     })
    /// }
    /// let a = [10, 20, 5, -23, 0];
    /// let b: [u32; 0] = [];
    ///
    /// assert_eq!(find_max(&mut a.into_gen()).unwrap(), Some(&20));
    /// assert_eq!(find_max(&mut b.into_gen()).unwrap(), None);
    /// ```
    ///
    /// With a stopping generator:
    ///
    /// ```
    /// use pushgen::{Generator, ValueResult, GeneratorResult, GeneratorExt};
    /// // This is a very very basic stopping generator, only for demonstration purposes!
    /// struct StoppingGen(i32, usize);
    /// impl Generator for StoppingGen {
    ///     type Output = i32;
    ///
    ///     fn run(&mut self, mut output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult{
    ///         static DATA: [i32;4] = [1, 2, 3, 4];
    ///         if self.0 == 0 {
    ///             self.0 = -1;
    ///             return GeneratorResult::Stopped;
    ///         }
    ///         while self.1 < DATA.len() {
    ///             if self.0 == 0 {
    ///                 self.0 = -1;
    ///                 return GeneratorResult::Stopped;
    ///             }
    ///             let res = output(DATA[self.1]);
    ///             self.0 -= 1;
    ///             self.1 += 1;
    ///             if res == ValueResult::Stop {
    ///                 return GeneratorResult::Stopped;
    ///             }
    ///         }
    ///         GeneratorResult::Complete
    ///     }
    /// }
    /// // Generator will produce `[1, *Stopped*, 2, 3, 4]`.
    /// let mut gen = StoppingGen(1, 0);
    /// let partial = gen.try_reduce(None, |a, b| a + b);
    /// assert!(partial.is_err());
    /// let partial = partial.unwrap_err();
    /// assert_eq!(partial, Some(1));
    /// let res = gen.try_reduce(partial, |a, b| a + b);
    /// assert!(res.is_ok());
    /// assert_eq!(res.unwrap(), Some(1+2+3+4));
    /// ```
    ///
    #[inline]
    fn try_reduce<F>(
        &mut self,
        prev_reduction: Option<Self::Output>,
        mut reducer: F,
    ) -> Result<Option<Self::Output>, Option<Self::Output>>
    where
        Self: Sized,
        F: FnMut(Self::Output, Self::Output) -> Self::Output,
    {
        let left_value = {
            if let Some(prev) = prev_reduction {
                prev
            } else {
                // Grab the first item into an optional
                let mut first = None;
                let run_result = self.run(|x| {
                    first = Some(x);
                    ValueResult::Stop
                });

                // In the hot loop we use an inplace updatable since we know we will never
                // have a None option from now on.
                match run_result {
                    GeneratorResult::Stopped => match first {
                        None => return Err(None),
                        Some(first) => first,
                    },
                    GeneratorResult::Complete => return Ok(first),
                }
            }
        };

        let mut left_value = crate::structs::utility::InplaceUpdatable::new(left_value);

        let run_result = self.run(|x| {
            left_value.inplace_reduce(x, &mut reducer);
            ValueResult::MoreValues
        });

        let result = Some(left_value.get_inner());

        match run_result {
            GeneratorResult::Stopped => Err(result),
            GeneratorResult::Complete => Ok(result),
        }
    }
}

impl<T: Generator> GeneratorExt for T {}

#[cfg(test)]
mod tests {
    use crate::test::StoppingGen;
    use crate::{Generator, GeneratorExt, GeneratorResult, IntoGenerator, ValueResult};

    #[test]
    fn for_each_stopped() {
        struct StoppingGen;
        impl Generator for StoppingGen {
            type Output = i32;

            fn run(&mut self, _output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
                GeneratorResult::Stopped
            }
        }

        let mut gen = StoppingGen;
        assert_eq!(gen.for_each(|_| ()), GeneratorResult::Stopped);
    }

    #[test]
    fn empty_all() {
        let data: [i32; 0] = [];
        assert!(data.into_gen().all(|_| false));
    }

    #[test]
    fn basic_all() {
        let data = [1, 2, 2];
        assert!(data.into_gen().all(|&x| x > 0));
        assert!(!data.into_gen().all(|&x| x > 2));
    }

    #[test]
    fn shortcircuit_all() {
        let data = [1, 2, 3];
        let mut gen = data.into_gen();
        assert!(!gen.all(|&x| x != 2));
        assert_eq!(gen.iter().next(), Some(&3));
    }

    #[test]
    fn empty_any() {
        let data: [i32; 0] = [];
        assert!(!data.into_gen().any(|_| true));
    }

    #[test]
    fn empty_reduce() {
        let x: [i32; 0] = [];
        fn reducer(a: i32, b: i32) -> i32 {
            a + b
        }

        assert_eq!(
            x.iter().copied().reduce(reducer),
            x.into_gen().copied().reduce(reducer)
        );
    }

    #[test]
    fn single_element_reduce() {
        let x = [1i32];
        fn reducer(a: i32, b: i32) -> i32 {
            a + b
        }

        assert_eq!(
            x.iter().copied().reduce(reducer),
            x.into_gen().copied().reduce(reducer)
        );
    }

    #[test]
    fn two_element_reduce() {
        let x = [1i32, 2];
        fn reducer(a: i32, b: i32) -> i32 {
            a + b
        }

        assert_eq!(
            x.iter().copied().reduce(reducer),
            x.into_gen().copied().reduce(reducer)
        );
    }

    #[test]
    fn empty_try_reduce() {
        let x: [i32; 0] = [];
        fn reducer(a: i32, b: i32) -> i32 {
            a + b
        }

        assert_eq!(x.into_gen().copied().try_reduce(None, reducer), Ok(None));
    }

    #[test]
    fn single_element_try_reduce() {
        let x = [1i32];
        fn reducer(a: i32, b: i32) -> i32 {
            a + b
        }

        assert_eq!(x.into_gen().copied().try_reduce(None, reducer), Ok(Some(1)));
    }

    #[test]
    fn two_element_try_reduce() {
        let x = [1i32, 2];
        fn reducer(a: i32, b: i32) -> i32 {
            a + b
        }

        assert_eq!(x.into_gen().copied().try_reduce(None, reducer), Ok(Some(3)));
    }

    #[test]
    fn stop_at_start_try_reduce() {
        for i in 0..4 {
            let x = [1, 2, 3, 4, 5];
            let mut gen = StoppingGen::new(i, &x);
            let res = gen.try_reduce(None, |a, b| match a < b {
                true => a,
                false => b,
            });

            assert!(res.is_err());
            let partial = res.unwrap_err();
            if i == 0 {
                assert_eq!(partial, None);
            } else {
                assert_eq!(partial, Some(&1));
            }
            match gen.try_reduce(partial, |a, b| if a < b { a } else { b }) {
                Ok(x) => assert_eq!(x, Some(&1)),
                Err(_) => {
                    assert!(false);
                }
            }
        }
    }
}
