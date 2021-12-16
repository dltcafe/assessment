//! # Assessment
//!
//! A library that allows different types of assessments, to convert between them and to perform
//! basic operations.
//!
//! Currently, the lib supports *numeric* and *interval* **quantitative** values
//! and *linguistic*, *2-tuple* and *hesitant* **fuzzy** values.
//!
//! Note that the library is a **Work In Progress**.

#![feature(trait_alias)]
#![macro_use]
extern crate impl_ops;

pub use domain::Domain;
pub use valuation::Valuation;

pub mod domain;
pub mod fuzzy;
pub mod utilities;
pub mod valuation;
