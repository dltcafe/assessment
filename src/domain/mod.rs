//! Domains in which assessments are represented

pub use qualitative::Qualitative;
pub use quantitative::Quantitative;

/// Quantitative struct and related implementations
pub mod quantitative;

/// Qualitative struct and related implementations
pub mod qualitative;

/// Base trait for domains
pub trait Domain {}
