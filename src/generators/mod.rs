//! Implements various generators.

pub use from_fn::from_fn;
pub use from_fn::FromFn;
pub use from_iter::from_iter;
pub use from_iter::FromIter;
pub use slice_generator::SliceGenerator;
pub use option::OptionGen;

mod slice_generator;
mod from_fn;
mod from_iter;
mod option;

#[cfg(feature = "std")]
mod boxed;
#[cfg(feature = "std")]
pub use boxed::BoxedGenerator;

