use crate::Generator;
use crate::generator::structs::{Transform, Filter, Skip, Take};

pub trait Sealed {}

impl<T> Sealed for T
where
    T: Generator
{}

pub trait GeneratorExt: Sealed + Generator {
    fn filter<Pred>(self, predicate: Pred) -> Filter<Self, Pred>
    where
        Self: Sized,
        Pred: FnMut(&Self::Output) -> bool {
        Filter::new(self, predicate)
    }

    fn transform<Trans, Out>(self, transform_fn: Trans) -> Transform<Self, Trans, Out>
    where
        Self: Sized,
        Trans: FnMut(Self::Output) -> Out {
        Transform::new(self, transform_fn)
    }

    fn skip(self, amount: usize) -> Skip<Self>
    where
        Self: Sized,
    {
        Skip::new(self, amount)
    }

    fn take(self, amount: usize)-> Take<Self>
    where
        Self: Sized
    {
        Take::new(self, amount)
    }
}

impl<T: Generator> GeneratorExt for T
{

}
