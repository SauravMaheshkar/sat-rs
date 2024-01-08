use crate::notation::Formula;
use std::collections::HashMap;

/// Chaos Algorithm for evaluation of propostional formulas
///
/// This algorithm generates random interpretations and checks if the formula is satisfied by the
/// interpretation. If the formula is satisfied, the algorithm returns true. If the formula is not
/// satisfied, the algorithm returns false.
///
/// # Pseudocode
/// ```text
/// procedure CHAOS(Set of Clauses S)
///
/// input: A set of clauses S
/// output: true if S is satisfiable, false otherwise
/// parameters: max_tries
///
/// begin
///   repeat max_tries times
///     create a random interpretation I
///     if I satisfies S
///      then return true else return false
/// end
/// ```
///
/// # Arguments
/// * `formula` - A [`Formula`] struct
/// * `max_tries` - The number of times to try to find a satisfying interpretation
///
/// # Returns
/// * `bool` - The value of the formula
///
/// # Examples
/// ```
/// use sat_rs::notation::{Literal, Clause, Formula};
/// use sat_rs::solvers::chaos;
///
/// let mut valid_clause = Clause::new();
/// let p = Literal{ value: 1, negated: false};
/// valid_clause.literals.push(p);
///
/// let mut invalid_clause = Clause::new();
/// let q = Literal{ value: 1, negated: true};
/// invalid_clause.literals.push(q);
///
/// let mut formula = Formula {
///     clauses: vec![valid_clause, invalid_clause],
///     literals: vec![Literal{ value: 1, negated: false}, Literal{ value: 1, negated: true}],
///     vars: vec![1],
///     num_clauses: 2,
///     num_vars: 1,
/// };
///
/// let result = chaos::chaos_algorithm(&mut formula, 10);
/// assert_eq!(result, false);
/// ```
pub fn chaos_algorithm(formula: &mut Formula, max_tries: u32) -> bool {
    let mut value: bool = true;

    for _ in 0..max_tries {
        // Create a random interpretation
        let mut interpretation: HashMap<i32, bool> = HashMap::new();

        for var in &formula.vars {
            interpretation.insert(*var, rand::random::<bool>());
        }

        // Check if the interpretation satisfies the formula
        if formula.evaluate(&interpretation) {
            value = true;
            println!(
                "Formula is satisfied by the interpretation: {:?}",
                interpretation
            );
            break;
        } else {
            value = false;
        }
    }

    return value;
}
