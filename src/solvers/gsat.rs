use crate::notation::{Clause, Formula};
use crate::solvers::utils::flip;
use rand::prelude::IndexedRandom;
use rand::Rng;
use std::collections::HashMap;

/// GSAT Algorithm for evaluation of propostional formulas
///
/// This algorithm generates random interpretations and checks if the formula is satisfied by the
/// interpretation. If the formula is satisfied, the algorithm returns true. If the formula is not
/// statisfied, the algorithm selects a variable which upon flipping satisfies the maximum number
/// of clauses. The algorithm then flips the value of the variable in the interpretation. The
/// algorithm then checks if the formula is satisfied by the new interpretation. If the formula is
/// satisfied, the algorithm returns true. If the formula is not satisfied, the algorithm repeats
/// the process until the formula is satisfied or the maximum number of flips is reached.
///
/// # Pseudocode
/// ```text
/// procedure GSAT(Set of Clauses S)
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
///         p := variable which upon flipping satisfies the maximum number of clauses
///         I = flip(I, p)
///         if I satisfies S
///           then return true
///         else
///           continue
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
/// ```rust
/// use sat_rs::notation::{Literal, Clause, Formula};
/// use sat_rs::solvers::gsat;
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
/// let result = gsat::gsat_algorithm(&mut formula, 10, 10, None);
/// assert_eq!(result, true);
/// ```
pub fn gsat_algorithm(
    formula: &mut Formula,
    max_tries: u32,
    max_flips: u32,
    walk_probability: Option<f32>,
) -> bool {
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
                // Collect unsatisfied clauses
                let unsatisfied_clauses: Vec<Clause> = formula.get_unsatisfied_clauses();

                // Loop over all the unsatisfied clauses, and find the variable
                // which upon flipping satisfies the maximum number of clauses
                let mut var_to_flip: i32 = 0;
                for clause in &unsatisfied_clauses {
                    let mut max_satisfied_clauses: u32 = 0;
                    for literal in &clause.literals {
                        let mut interpretation_copy = interpretation.clone();
                        interpretation_copy = flip(&mut interpretation_copy, literal.value).clone();

                        let mut num_satisfied_clauses: u32 = 0;
                        // Clone the formual per iteration
                        let mut temp_formula = formula.clone();

                        for clause in &mut temp_formula.clauses {
                            clause.is_satisfied = clause.evaluate(&interpretation_copy);
                            if clause.is_satisfied {
                                num_satisfied_clauses += 1;
                            }
                        }

                        if num_satisfied_clauses > max_satisfied_clauses {
                            max_satisfied_clauses = num_satisfied_clauses;
                            var_to_flip = literal.value;
                        }
                    }
                }

                // Flip the value of the variable
                if walk_probability == None {
                    interpretation = flip(&mut interpretation, var_to_flip).clone();
                } else {
                    // flip the value of the variable with a probability walk_probability
                    // and flip a random variable with probability 1 - walk_probabilityN
                    let random_number: f32 = rand::random::<f32>();
                    if random_number < walk_probability.unwrap() {
                        interpretation = flip(&mut interpretation, var_to_flip).clone();
                    } else {
                        // Randomly select a clause that is not satisfied by the interpretation
                        let clause = unsatisfied_clauses.choose(&mut rand::rng());
                        let clausal_variables: Vec<i32> =
                            formula.get_clausal_variables(&clause.unwrap());
                        let random_var =
                            clausal_variables[rand::rng().random_range(0..clausal_variables.len())];
                        interpretation = flip(&mut interpretation, random_var).clone();
                    }
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
                    continue;
                }
            }
        }
    }

    return value;
}
