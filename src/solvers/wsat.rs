use crate::notation::{Clause, Formula};
use crate::solvers::utils::flip;
use rand::prelude::IndexedRandom;
use rand::RngExt;
use std::collections::HashMap;

/// WSAT Algorithm for evaluation of propostional formulas
///
/// This algorithm generates random interpretations and checks if the formula is satisfied by the
/// interpretation. If the formula is satisfied, the algorithm returns true. If the formula is not
/// satisfied, the algorithm selects a random unsatisfied clause and a random variable from the
/// clause and flips the value of the variable in the interpretation. The algorithm then checks if
/// the formula is satisfied by the new interpretation. If the formula is satisfied, the algorithm
/// returns true. If the formula is not satisfied, the algorithm repeats the process until the
/// formula is satisfied or the maximum number of flips is reached.
///
/// # Pseudocode
/// ```text
/// procedure WSAT(Set of Clauses S)
///
/// input: A set of clauses S
/// output: true if S is satisfiable, false otherwise
/// parameters: max_tries, max_flips
///
/// begin
///   repeat max_tries times
///     create a random interpretation I
///     if I satisfies S
///       then return true
///     else
///       repeat max_flips times
///         randomly select an unsatisfied clause C
///         randomly select a variable p from C
///         flip the value of p in I
///         if I satisfies S
///           then return true
///         else
///          continue
/// end
/// ```
///
/// # Arguments
/// * `formula` - A [`Formula`] struct
/// * `max_tries` - The number of times to try to find a satisfying interpretation
/// * `max_flips` - The number of times to flip the value of a variable in an interpretation
///
/// # Returns
/// * `bool` - The value of the formula
///
/// # Examples
/// ```
/// use sat_rs::notation::{Literal, Clause, Formula};
/// use sat_rs::solvers::wsat;
///
/// let mut clause = Clause::new(); // Clause: p v q
/// let p = Literal{ value: 1, negated: false};
/// let q = Literal{ value: 2, negated: false};
/// clause.literals.push(p);
/// clause.literals.push(q);
///
/// let mut formula = Formula {
///    clauses: vec![clause],
///    literals: vec![Literal{ value: 1, negated: false}, Literal{ value: 2, negated: false}],
///    vars: vec![1, 2],
///    num_clauses: 1,
///    num_vars: 2,
/// };
///
/// let result = wsat::wsat_algorithm(&mut formula, 10, 10);
/// assert_eq!(result, true);
/// ```
pub fn wsat_algorithm(formula: &mut Formula, max_tries: u32, max_flips: u32) -> bool {
    let mut value: bool = false;

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
            return value;
        } else {
            for _ in 0..max_flips {
                // Randomly select a clause that is not satisfied by the interpretation
                let unsatisfied_clauses: Vec<Clause> = formula.get_unsatisfied_clauses();
                let clause = unsatisfied_clauses.choose(&mut rand::rng());

                // Randomly select a variable from the clause
                let clausal_variables: Vec<i32> = formula.get_clausal_variables(&clause.unwrap());
                let variable =
                    clausal_variables[rand::rng().random_range(0..clausal_variables.len())];

                interpretation = flip(&mut interpretation, variable).clone();

                // Check if the interpretation satisfies the formula
                if formula.evaluate(&interpretation) {
                    value = true;
                    println!(
                        "Formula is satisfied by the interpretation: {:?}",
                        interpretation
                    );
                    return value;
                } else {
                    continue;
                }
            }
        }
    }

    return value;
}
