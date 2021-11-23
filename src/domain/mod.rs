//! Domains in which assessments are represented

pub use quantitative::Quantitative;

/// Quantitative struct and related implementations
pub mod quantitative;

/// Base trait for domains
pub trait Domain {}
