//! # SAT Solvers in Rust
//! This crate contains implementations of various satisfiability solvers for the boolean satisfiability problem (SAT).
//!
//! List of available solvers:
//! * [`crate::solvers::syntactic`] - A purely syntactic solver based on a user provided interpretation
//!
//! This crate also contains some useful structs for working with propositional variables and formulas, viz:
//! * [`crate::notation::Formula`] - A struct for working with propositional formulas
//! * [`crate::notation::Clause`] - A struct for working with propositional clauses
//! * [`crate::notation::Literal`] - A struct for working with propositional literals (atoms)
//!
//! # Usage
//! The crate can be used as a library or as a binary. To use it as a binary, run the following command:
//! ```text
//! cargo run <CNF_FILE> <SOLVER>
//! ```
//! OR
//! ```text
//! sat-rs <CNF_FILE> <SOLVER>
//! ```
pub mod notation;
pub mod cnfparser;
pub mod solvers;
