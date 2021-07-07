//! Generator adaptor implementations. See [`GeneratorExt`](crate::GeneratorExt) for more info.

mod chain;
mod dedup;
mod filter;
mod filter_map;
mod flatten;
pub(crate) mod from_fn;
mod iterator;
mod map;
mod skip;
mod take;
mod zip;

pub use chain::Chain;
pub use dedup::Dedup;
pub use filter::Filter;
pub use filter_map::FilterMap;
pub use flatten::Flatten;
pub use from_fn::FromFn;
pub use iterator::IteratorAdaptor;
pub use map::Map;
pub use skip::Skip;
pub use take::Take;
pub use zip::Zip;
