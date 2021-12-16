//! Valuations used for assessments.

pub use interval::{Interval, IntervalError};
pub use linguistic::{
    Hesitant, HesitantError, HesitantRelation, Linguistic, Single, SingleError, TwoTuple,
    TwoTupleError,
};
pub use numeric::{Numeric, NumericError};
pub use unification::*;

/// Interval struct and related implementations.
pub mod interval;

/// Linguistic trait and valuations implementations.
pub mod linguistic;

/// Numeric struct and related implementations.
pub mod numeric;

/// Implementations for unification.
pub mod unification;

/// Base trait for valuations.
pub trait Valuation {}
