//! Helper traits for various operations.
//!
//! These traits are generally not used outside of the library but can be implemented to enable
//! certain operations for custom types.

mod accum;

pub use accum::Product;
pub use accum::Sum;
