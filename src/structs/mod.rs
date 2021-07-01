//! Generator adaptor implementations. See [`GeneratorExt`](crate::GeneratorExt) for more info.

mod skip;
mod filter;
mod map;
mod take;
mod chain;
mod zip;
mod dedup;
mod flatten;

pub use map::Map;
pub use skip::Skip;
pub use filter::Filter;
pub use take::Take;
pub use chain::Chain;
pub use zip::Zip;
pub use dedup::Dedup;
pub use flatten::Flatten;