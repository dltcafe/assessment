//! Valuations used for assessments.

pub use interval::{Interval, IntervalError};
pub use linguistic::{Linguistic, Single, SingleError, TwoTuple, TwoTupleError};
pub use numeric::{Numeric, NumericError};

/// Interval struct and related implementations.
pub mod interval;

/// Linguistic trait and valuations implementations.
pub mod linguistic;

/// Numeric struct and related implementations.
pub mod numeric;

/// Base trait for valuations.
pub trait Valuation {}
