//! Valuations used for assessments.

pub use interval::Interval;
//pub use numeric::Numeric;
//pub use numeric::Value as NumericValue;

/// Numeric struct and related implementations.
//pub mod numeric;

/// Interval struct and related implementations.
pub mod interval;

/// Base trait for valuations.
pub trait Valuation {}
