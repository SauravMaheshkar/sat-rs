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
pub fn interactive_algorithm(formula: &mut Formula) -> Result<bool, Error>{
    // Ask for an interpretation
    let mut interpretation: HashMap<i32, bool> = HashMap::new();

    for literal in &formula.literals {
        println!("Enter value for literal {} in the interpretation: ", literal);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input: bool = input.trim().parse().unwrap();
        interpretation.insert(*literal, input);
    }

    // Evaluate formula based on the interpretation
    let value = formula.evaluate(&interpretation);

    return Ok(value);
}
