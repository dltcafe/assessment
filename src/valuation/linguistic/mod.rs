use crate::Valuation;

pub use single::{Single, SingleError};
pub use two_tuple::{TwoTuple, TwoTupleError};

/// Single linguistic valuations.
pub mod single;

/// TwoTuple linguistic valuations.
pub mod two_tuple;

/// Linguistic valuations.
pub trait Linguistic {}

impl Valuation for dyn Linguistic {}
