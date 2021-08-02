use crate::GeneratorExt;
use crate::IntoGenerator;

/// Conversion from a [`Generator`].
///
/// By implementing `FromGenerator` for a type, you define how it will be created from a generator.
/// This is common for types which describe a collection of some kind.
///
/// [`FromGenerator::from_gen`] is rarely called explicitly, but is instead used through [`GeneratorExt::collect()`].
///
/// [`GeneratorExt::collect()`]: crate::GeneratorExt::collect
/// [`Generator`]: crate::Generator
///
/// ## Examples
///
/// Basic usage:
///
/// ```
/// use pushgen::{FromGenerator, IntoGenerator, GeneratorExt};
/// let v: Vec<i32> = FromGenerator::from_gen([1, 2, 3, 4].into_gen().copied());
/// assert_eq!(v, [1, 2, 3, 4]);
/// ```
///
/// Using [`GeneratorExt::collect()`] to implicitly use `FromGenerator`:
///
/// ```
/// use pushgen::{FromGenerator, IntoGenerator, GeneratorExt};
/// let v: Vec<i32> = [1, 2, 3, 4].into_gen().copied().collect();
/// assert_eq!(v, [1, 2, 3, 4]);
/// ```
///
/// Implementing `FromGenerator` for your type:
///
/// ```
/// use pushgen::{FromGenerator, IntoGenerator, GeneratorExt};
///
/// // A sample collection, that's just a wrapper over Vec<T>
/// #[derive(Debug)]
/// struct MyCollection(Vec<i32>);
///
/// // Let's give it some methods so we can create one and add things
/// // to it.
/// impl MyCollection {
///     fn new() -> MyCollection {
///         MyCollection(Vec::new())
///     }
///
///     fn add(&mut self, elem: i32) {
///         self.0.push(elem);
///     }
/// }
///
/// // and we'll implement FromIterator
/// impl FromGenerator<i32> for MyCollection {
///     fn from_gen<G: IntoGenerator<Output=i32>>(gen: G) -> Self {
///         let mut c = MyCollection::new();
///
///         gen.into_gen().for_each(|x| c.add(x));
///
///         c
///     }
/// }
///
/// // Now we can make a new iterator...
/// let gen = [0, 1, 2, 3, 4].into_gen().copied();
///
/// // ... and make a MyCollection out of it
/// let c = MyCollection::from_gen(gen);
///
/// assert_eq!(c.0, vec![0, 1, 2, 3, 4]);
///
/// // collect works too!
///
/// let gen = [0, 1, 2, 3, 4].into_gen().copied();
/// let c: MyCollection = gen.collect();
///
/// assert_eq!(c.0, vec![0, 1, 2, 3, 4]);
/// ```
///
pub trait FromGenerator<A> {
    /// Creates a value from a generator.
    fn from_gen<G>(gen: G) -> Self
    where
        G: IntoGenerator<Output = A>;
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl<T> FromGenerator<T> for Vec<T> {
    #[inline]
    fn from_gen<G>(gen: G) -> Self
    where
        G: IntoGenerator<Output = T>,
    {
        let mut ret = Self::new();
        gen.into_gen().for_each(|x| ret.push(x));
        ret
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl FromGenerator<char> for String {
    #[inline]
    fn from_gen<G>(gen: G) -> Self
    where
        G: IntoGenerator<Output = char>,
    {
        let mut ret = Self::new();
        gen.into_gen().for_each(|x| ret.push(x));
        ret
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl<'a> FromGenerator<&'a char> for String {
    #[inline]
    fn from_gen<G>(gen: G) -> Self
    where
        G: IntoGenerator<Output = &'a char>,
    {
        let mut ret = Self::new();
        gen.into_gen().for_each(|x| ret.push(*x));
        ret
    }
}
