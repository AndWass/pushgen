use crate::transform::Transform;
use crate::{End, Filter, InputOutputStage, Then, Take, Skip};

pub trait StageExt: InputOutputStage {
    fn take(self, amount: usize) -> Then<Self, Take<Self::Output>>
    where
        Self: Sized
    {
        Then::new(self, Take::new(amount))
    }

    fn skip(self, amount: usize) -> Then<Self, Skip<Self::Output>>
        where
            Self: Sized
    {
        Then::new(self, Skip::new(amount))
    }

    fn filter<P>(self, predicate: P) -> Then<Self, Filter<P, Self::Output>>
    where
        Self: Sized,
        P: FnMut(&Self::Output) -> bool,
    {
        Then::new(self, Filter::new(predicate))
    }

    fn transform<T, R>(self, transform: T) -> Then<Self, Transform<Self::Output, R, T>>
    where
        Self: Sized,
        T: FnMut(&Self::Output) -> R,
    {
        Then::new(self, Transform::new(transform))
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
