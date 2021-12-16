//! Domains in which assessments are represented.

pub use qualitative::{Qualitative, QualitativeError};
pub use quantitative::{Quantitative, QuantitativeError, QuantitativeLimit};

/// Quantitative struct and related implementations.
pub mod quantitative;

/// Qualitative struct and related implementations.
pub mod qualitative;

/// Domain factories
pub mod factories;

/// Base trait for domains.
pub trait Domain {}
