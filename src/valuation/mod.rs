//! Valuations used for assessments.

pub use interval::{Interval, IntervalError};
pub use linguistic::{Linguistic, Single, SingleError};
pub use numeric::{Numeric, NumericError};

/// Numeric struct and related implementations.
pub mod numeric;

/// Interval struct and related implementations.
pub mod interval;

/// Linguistic trait and valuations implementations.
pub mod linguistic;

/// Base trait for valuations.
pub trait Valuation {}
