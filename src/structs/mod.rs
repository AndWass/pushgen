//! Generator adaptor implementations. See [`GeneratorExt`](crate::GeneratorExt) for more info.

pub use chain::Chain;
pub use cloned::Cloned;
pub use copied::Copied;
pub use dedup::Dedup;
pub use filter::Filter;
pub use filter_map::FilterMap;
pub use flatten::Flatten;
pub use iterator::IteratorAdaptor;
pub use map::Map;
pub use skip::{Skip, SkipWhile};
pub use take::{Take, TakeWhile};
pub use zip::Zip;

mod chain;
mod cloned;
mod copied;
mod dedup;
mod filter;
mod filter_map;
mod flatten;
mod iterator;
mod map;
mod skip;
mod take;
pub(crate) mod utility;
mod zip;

