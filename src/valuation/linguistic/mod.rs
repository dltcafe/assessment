use crate::Valuation;

pub use single::{Single, SingleError};

/// Single linguistic valuations
pub mod single;

/// Linguistic valuations
pub trait Linguistic {}

impl Valuation for dyn Linguistic {}
