//! Implements various generators.

pub use array_gen::ArrayGenerator;
pub use from_fn::from_fn;
pub use from_fn::FromFn;
pub use from_iter::from_iter;
pub use from_iter::FromIter;
pub use option::OptionGen;
pub use slice_generator::SliceGenerator;

mod array_gen;
mod from_fn;
mod from_iter;
mod option;
mod slice_generator;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
mod boxed;
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub use boxed::BoxedGenerator;
