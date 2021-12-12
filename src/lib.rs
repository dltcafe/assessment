//! # Assessment
//!
//! A library that allows different types of assessments, to convert between them and to perform
//! basic operations.
//!
//! Currently, the lib supports *discrete* and *interval* **quantitative** values (in both cases
//! using integers and decimals values) and *linguistic*, *2-tuple* and *hesitant* **fuzzy** values.
//!
//! Note that the library is a **Work In Progress** and is **NOT READY YET**.

#![feature(trait_alias)]
#![macro_use]
extern crate impl_ops;

pub use domain::Domain;
pub use valuation::Valuation;

pub mod domain;
pub mod fuzzy;
pub mod valuation;
