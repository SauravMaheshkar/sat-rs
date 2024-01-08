//! # Utility Functions
//!
//! Utility functions for the solvers.
//!
//! ## Contents
//!
//! * [`flip`] - Flips the value of a variable in an interpretation.
use std::collections::HashMap;

/// Flips the value of a variable in an interpretation
///
/// # Arguments
/// * `interpretation` - The interpretation to flip the value of a variable in, of type [`HashMap<i32, bool>`].
/// * `variable` - The variable to flip the value of, of type [`i32`].
///
/// # Returns
/// * `interpretation` - The interpretation with the flipped value of the variable.
///
/// # Examples
/// ```
/// use std::collections::HashMap;
/// use sat_rs::solvers::utils::flip;
///
/// let mut interpretation: HashMap<i32, bool> = HashMap::new();
/// interpretation.insert(1, true);
///
/// flip(&mut interpretation, 1);
///
/// assert_eq!(interpretation.get(&1), Some(&false));
/// ```
pub fn flip(interpretation: &mut HashMap<i32, bool>, variable: i32) -> &mut HashMap<i32, bool> {
    for (key, value) in interpretation.iter_mut() {
        if *key == variable {
            *value = !*value;
        }
    }

    return interpretation;
}
