use crate::Valuation;

pub use hesitant::{Hesitant, HesitantError, HesitantRelation};
pub use single::{Single, SingleError};
pub use two_tuple::{TwoTuple, TwoTupleError};

/// Single linguistic valuations.
pub mod single;

/// TwoTuple linguistic valuations.
pub mod two_tuple;

/// Hesitant linguistic valuations.
pub mod hesitant;

/// Linguistic valuations.
pub trait Linguistic {}

impl Valuation for dyn Linguistic {}
