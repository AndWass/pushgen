pub trait IntoGenerator {
    /// Data-type generated by the generator.
    type Output;

    /// Which kind of generator are we turning this into?
    type IntoGen: crate::Generator<Output = Self::Output>;

    /// Creates a generator from a value.
    ///
    /// See the [module-level documentation] for more.
    ///
    /// [module-level documentation]: crate
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use pushgen::IntoGenerator;
    /// use crate::pushgen::GeneratorExt;
    /// let v = vec![1, 2, 3];
    /// let mut gen = v.into_gen();
    ///
    /// let mut output: Vec<i32> = Vec::new();
    /// gen.for_each(|x| output.push(x));
    /// assert_eq!(output, [1, 2, 3]);
    /// ```
    fn into_gen(self) -> Self::IntoGen;
}

impl<G: crate::Generator> IntoGenerator for G {
    type Output = G::Output;
    type IntoGen = G;
    #[inline]
    fn into_gen(self) -> Self::IntoGen {
        self
    }
}

impl<'a, T> IntoGenerator for &'a [T] {
    type Output = &'a T;
    type IntoGen = crate::SliceGenerator<'a, T>;
    #[inline]
    fn into_gen(self) -> Self::IntoGen {
        crate::SliceGenerator::new(self)
    }
}

impl<'a, T, const N: usize> IntoGenerator for &'a [T; N] {
    type Output = &'a T;
    type IntoGen = crate::SliceGenerator<'a, T>;
    #[inline]
    fn into_gen(self) -> Self::IntoGen {
        crate::SliceGenerator::new(self)
    }
}

#[cfg(feature = "std")]
impl<'a, T> IntoGenerator for &'a Vec<T> {
    type Output = &'a T;
    type IntoGen = crate::SliceGenerator<'a, T>;
    #[inline]
    fn into_gen(self) -> Self::IntoGen {
        crate::SliceGenerator::new(self.as_slice())
    }
}

#[cfg(feature = "std")]
impl<T> IntoGenerator for Vec<T> {
    type Output = T;
    type IntoGen = crate::generators::FromIter<std::vec::IntoIter<T>>;
    #[inline]
    fn into_gen(self) -> Self::IntoGen {
        crate::from_iter(self)
    }
}

impl<T> IntoGenerator for Option<T> {
    type Output = T;
    type IntoGen = crate::generators::OptionGen<T>;
    #[inline]
    fn into_gen(self) -> Self::IntoGen {
        crate::generators::OptionGen::new(self)
    }
}

impl<'t, T> IntoGenerator for &'t Option<T> {
    type Output = &'t T;
    type IntoGen = crate::generators::OptionGen<&'t T>;
    #[inline]
    fn into_gen(self) -> Self::IntoGen {
        crate::generators::OptionGen::new(self.as_ref())
    }
}