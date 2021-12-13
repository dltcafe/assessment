use crate::Valuation;

pub use hesitant::{Hesitant, HesitantError, HesitantRelation};
pub use single::{Single, SingleError};
pub use two_tuple::{TwoTuple, TwoTupleError};
pub use unified::{Unified, UnifiedError};

/// Single linguistic valuations.
pub mod single;

/// TwoTuple linguistic valuations.
pub mod two_tuple;

/// Hesitant linguistic valuations.
pub mod hesitant;

/// Unified linguistic valuations.
pub mod unified;

/// Linguistic valuations.
pub trait Linguistic {}

impl Valuation for dyn Linguistic {}
