use crate::{structs::utility::InplaceUpdatable, Generator, ValueResult};
use core::option::Option::Some;

/// Adapt a generator into an iterator. See [`.iter()`](crate::GeneratorExt::iter) for more info.
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
    pub fn new(source: Src) -> Self {
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
        let mut retval = None;
        self.source.run(|x| {
            retval = Some(x);
            ValueResult::Stop
        });
        retval
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
    fn fold() {
        let data = [1, 2, 3, 4, 5];

        let sum = SliceGenerator::new(&data)
            .iter()
            .fold(0i32, |acc, elem| acc + elem);

        assert_eq!(sum, data.iter().sum())
    }
}
