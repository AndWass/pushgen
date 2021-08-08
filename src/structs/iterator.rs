use crate::{
    structs::utility::InplaceUpdatable, Generator, GeneratorExt, ReverseGenerator, ValueResult,
};

/// Adapt a generator into an iterator. See [`.iter()`](crate::GeneratorExt::iter) for more info.
#[derive(Clone)]
pub struct IteratorAdaptor<Src>
where
    Src: Generator,
{
    source: Src,
}

impl<Src> IteratorAdaptor<Src>
where
    Src: Generator,
{
    #[inline]
    pub(crate) fn new(source: Src) -> Self {
        Self { source }
    }
}

impl<Src> Iterator for IteratorAdaptor<Src>
where
    Src: Generator,
{
    type Item = Src::Output;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.source.next().ok()
    }

    #[inline]
    fn fold<B, F>(mut self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        let mut result = InplaceUpdatable::new(init);
        self.source.run(|x| {
            result.update(|val| f(val, x));
            ValueResult::MoreValues
        });
        result.get_inner()
    }
}

impl<Src> DoubleEndedIterator for IteratorAdaptor<Src>
where
    Src: ReverseGenerator,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.source.next_back().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{GeneratorExt, SliceGenerator};

    #[test]
    fn iter_over_slice() {
        let data = [1, 2, 3, 4, 5];
        let mut sum = 0;
        for x in IteratorAdaptor::new(SliceGenerator::new(&data)) {
            sum += x;
        }

        assert_eq!(sum, data.iter().sum());
    }

    #[test]
    fn reverse_iter() {
        let data = [1, 2, 3, 4];
        let mut out = Vec::new();
        for x in IteratorAdaptor::new(SliceGenerator::new(&data)).rev() {
            out.push(*x);
        }

        assert_eq!(out, [4, 3, 2, 1]);
    }

    #[test]
    fn fold() {
        let data = [1, 2, 3, 4, 5];

        let sum = SliceGenerator::new(&data)
            .iter()
            .fold(0i32, |acc, elem| acc + elem);

        assert_eq!(sum, data.iter().sum())
    }
}
