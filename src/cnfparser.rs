//! DIMACS CNF Parser
//!
//! parser for DIMACS CNF files, returns a [`Formula`] struct

use crate::notation::{Clause, Formula, Literal};

/// Parses a CNF file and returns a [`Formula`] struct
///
/// # Arguments
/// * `_buffer` - A string slice ([`str`]) that holds the contents of the CNF file
///
/// # Returns
/// * [`Formula`] - A [`Formula`] struct
///
/// # Raises
/// * [`String`] - An error message
pub fn parse_cnf(_buffer: &str) -> Result<Formula, String> {
    let mut num_clauses: i32 = 0;
    let mut num_vars: i32 = 0;
    let mut clauses: Vec<Clause> = Vec::new();

    // Iterate over lines in the buffer
    for line in _buffer.lines() {
        // Skip comments
        if line.starts_with("c") {
            continue;
        } else if line.starts_with("p") {
            // If the line starts with "p", then it contains the number of clauses and variables
            let mut tokens = line.split_whitespace();

            // Skip the first two tokens, i.e. "p" and "cnf" if the file is valid
            for _ in 0..2 {
                if tokens.next().is_none() {
                    // Handle the case where there are not enough tokens
                    return Err("Not enough tokens in the line".to_string());
                }
            }

            // Get the number of variables
            let num_vars_str = tokens.next().ok_or("Missing num_vars token")?;
            num_vars = num_vars_str
                .parse::<i32>()
                .map_err(|e| format!("Failed to parse num_vars: {}", e))?;

            // Get the number of clauses
            let num_clauses_str = tokens.next().ok_or("Missing num_clauses token")?;
            num_clauses = num_clauses_str
                .parse::<i32>()
                .map_err(|e| format!("Failed to parse num_clauses: {}", e))?;
        } else {
            // If the line does not start with "c" or "p", then it contains a clause
            let mut clause: Clause = Clause::new();

            // Iterate over tokens in the line
            for token in line.split_whitespace() {
                let _lit = token.parse::<i32>().unwrap();
                if _lit == 0 {
                    // If the token is 0, then it is the end of the clause
                    break;
                }
                let literal = Literal {
                    value: _lit.abs(),
                    negated: _lit < 0,
                };

                clause.literals.push(literal);
            }
            clauses.push(clause);
        }
    }

    // Get unique literals from the formula
    let mut literals: Vec<Literal> = Vec::new();
    for clause in &clauses {
        for literal in &clause.literals {
            if !literals.contains(literal) {
                literals.push(literal.clone());
            }
        }
    }

    // Get unique variables from the formula
    let mut vars: Vec<i32> = Vec::new();
    for clause in &clauses {
        for literal in &clause.literals {
            if !vars.contains(&literal.value) {
                vars.push(literal.value);
            }
        }
    }

    assert_eq!(num_clauses, clauses.len() as i32);
    assert_eq!(num_vars, vars.len() as i32);

    let formula: Formula = Formula {
        clauses,
        literals,
        vars,
        num_clauses,
        num_vars,
    };

    Ok(formula)
}
