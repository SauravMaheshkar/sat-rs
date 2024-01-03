use notation::Formula;
use std::{collections::HashMap, io::Error};

/// Purely Syntactic Algorithm for evaluation of propostional formulas
///
/// This algorithm requires the user to provide an interpretation for each literal in the formula
/// via the command line. The interpretation is then used to evaluate the formula.
///
/// # Psuedocode
/// ```text
/// procedure syntactic_algorithm(formula A, interpretation I)
///
/// input: A formula A, an interpretation I
/// output: the Boolean value I(A)
///
/// begin
///   forall atoms p occuring in A
///     if I |= p
///       then replace all occurences of p in A by true
///       else replace all occurences of p in A by false
///     rewrite A into a normal form using the rewrite rules
///   if A is a tautology then return true else return false
/// end
/// ```
///
/// # Arguments
/// * `formula` - A [`Formula`] struct
///
/// # Returns
/// * `Ok(bool)` - If the formula is satisfiable, returns `Ok(true)`, otherwise `Ok(false)`
/// * `Err(Error)` - If an error occurs, returns `Err(Error)`
///
/// # Examples
/// Assuming the CNF file is in `/bin/problem.cnf` and contains the following:
/// ```;
/// p cnf 3 1
/// 1 -3 0
/// 2 3 -1 0
/// ```
///
/// ```rust
/// use sat_rs::cnfparser;
/// use std::collections::HashMap;
///
/// let buffer = include_str!("bin/problem.cnf");
/// let mut formula = cnfparser::parse_cnf(&buffer);
///
/// let mut interpretation: HashMap<i32, bool> = HashMap::new();
/// interpretation.insert(1, false);
/// interpretation.insert(2, false);
/// interpretation.insert(3, false);
///
/// assert_eq!(formula.unwrap().evaluate(&interpretation), true);
/// ```
pub fn syntactic_algorithm(formula: Formula) -> Result<bool, Error>{
    // Create a mutable copy of the formula
    let mut _formula = formula.clone();

    // Ask for an interpretation
    let mut interpretation: HashMap<i32, bool> = HashMap::new();

    for literal in &formula.literals {
        println!("Enter value for literal {} in the interpretation: ", literal);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input: bool = input.trim().parse().unwrap();
        interpretation.insert(*literal, input);
    }

    for clause in &mut _formula.clauses {
        for literal in &mut clause.literals {
            if interpretation.contains_key(&literal.value) {
                if interpretation[&literal.value] {
                    literal.negated = false;
                } else {
                    literal.negated = true;
                }
            }
        }
    }

    let mut value: bool = true;

    for clause in &mut _formula.clauses {
        let mut _clausal_value: bool = true;
        for literal in &mut clause.literals {
            if literal.negated {
               _clausal_value = false;
            }
        }
        value = value && _clausal_value;
    }

    Ok(value)
}
