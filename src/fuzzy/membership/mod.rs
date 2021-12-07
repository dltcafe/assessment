//! Membership functions capture the degree of truth in a given value.

pub use trapezoidal::{Trapezoidal, TrapezoidalError};

/// Trapezoidal membership functions.
pub mod trapezoidal;

/// Base trait for memberships functions.
pub trait Membership {}
