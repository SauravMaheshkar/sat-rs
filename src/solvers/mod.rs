//! # Solvers
//!
//! This module contains implementations of various solvers for the SAT problem.
//!
//! ## Available solvers
//!
//! * [`interactive`] - A purely syntactic solver based on user provided input.
//! * [`chaos`] - A solver based on the chaos algorithm.
//! * [`wsat`] - A solver based on the wsat algorithm.
pub mod chaos;
pub mod interactive;
pub mod utils;
pub mod wsat;
