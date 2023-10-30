use std::fmt::Debug;

use notation::clauses::Clause;

pub struct Problem {
    clauses: Vec<Clause>,
    num_clauses: u32,
    num_variables: u32,
}

impl Problem {
    pub fn new(clauses: Vec<Clause>, num_clauses: u32, num_variables: u32) -> Problem {
        Problem {
            clauses: clauses,
            num_clauses: num_clauses,
            num_variables: num_variables,
        }
    }
}

impl Debug for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Problem {{ clauses: {:?}, num_clauses: {:?}, num_variables: {:?} }}",
            self.clauses, self.num_clauses, self.num_variables
        )
    }
}
