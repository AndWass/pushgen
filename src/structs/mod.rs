//! Generator adaptor implementations. See [`GeneratorExt`](crate::GeneratorExt) for more info.

#[cfg(feature = "std")]
pub(crate) mod boxed;

mod chain;
mod cloned;
mod copied;
mod dedup;
mod filter;
mod filter_map;
mod flatten;
pub(crate) mod from_fn;
pub(crate) mod from_iter;
mod iterator;
mod map;
mod option;
mod skip;
mod take;
mod utility;
mod zip;

pub use chain::Chain;
pub use cloned::Cloned;
pub use copied::Copied;
pub use dedup::Dedup;
pub use filter::Filter;
pub use filter_map::FilterMap;
pub use flatten::Flatten;
pub use from_fn::FromFn;
pub use from_iter::FromIter;
pub use iterator::IteratorAdaptor;
pub use map::Map;
pub use option::OptionGen;
pub use skip::Skip;
pub use take::Take;
pub use zip::Zip;
