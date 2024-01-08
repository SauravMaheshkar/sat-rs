//! # Solvers
//!
//! This module contains implementations of various solvers for the SAT problem.
//!
//! ## Available solvers
//!
//! * [`interactive`] - A purely syntactic solver based on user provided input.
//! * [`chaos`] - A solver based on the CHAOS algorithm.
//! * [`wsat`] - A solver based on the WSAT algorithm.
//! * [`gsat`] - A solver based on the GSAT algorithm.
pub mod chaos;
pub mod gsat;
pub mod interactive;
pub mod utils;
pub mod wsat;
