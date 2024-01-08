use crate::notation::Formula;
use std::collections::HashMap;

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
/// * `bool` - The value of the formula
pub fn interactive_algorithm(formula: &mut Formula) -> bool {
    let mut interpretation: HashMap<i32, bool> = HashMap::new();

    for var in &formula.vars {
        // Ask for an interpretation
        println!("Enter value for variable {:?} in the interpretation: ", var);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input: bool = input.trim().parse().unwrap();
        interpretation.insert(*var, input);
    }

    // Evaluate formula based on the interpretation
    let value = formula.evaluate(&interpretation);

    return value;
}
