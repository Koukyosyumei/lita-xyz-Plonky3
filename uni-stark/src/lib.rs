//! A minimal univariate STARK framework.

#![no_std]

extern crate alloc;

mod config;
mod decompose;
mod folder;
mod proof;
mod prover;
pub mod public;
pub mod symbolic_builder;
pub mod symbolic_expression;
pub mod symbolic_variable;
mod verifier;
mod zerofier_coset;

#[cfg(debug_assertions)]
mod check_constraints;

pub use config::*;
pub use decompose::*;
pub use folder::*;
pub use proof::*;
pub use prover::*;
pub use public::*;
pub use verifier::*;
pub use zerofier_coset::*;
