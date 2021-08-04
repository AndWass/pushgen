//! Generator adaptor implementations. See [`GeneratorExt`](crate::GeneratorExt) for more info.

pub use chain::Chain;
pub use cloned::Cloned;
pub use copied::Copied;
pub use dedup::Dedup;
pub use enumerate::Enumerate;
pub use filter::Filter;
pub use filter_map::FilterMap;
pub use flatten::Flatten;
pub use inspect::Inspect;
pub use iterator::IteratorAdaptor;
pub use map::Map;
pub use rev::Reverse;
pub use skip::{Skip, SkipWhile};
pub use step_by::StepBy;
pub use take::{Take, TakeWhile};
pub use zip::Zip;

mod chain;
mod cloned;
mod copied;
mod dedup;
mod enumerate;
mod filter;
mod filter_map;
mod flatten;
mod inspect;
mod iterator;
mod map;
mod rev;
mod skip;
mod step_by;
mod take;
pub(crate) mod utility;
mod zip;
