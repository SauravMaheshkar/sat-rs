use std::fmt::Debug;

use notation::variables::Variable;

pub struct Literal {
    variable: Variable,
    negated: bool,
}

impl Literal {
    pub fn new(variable: Variable, negated: bool) -> Literal {
        Literal {
            variable: variable,
            negated: negated,
        }
    }
}

impl Debug for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Literal {{ variable: {:?}, negated: {:?} }}",
            self.variable, self.negated
        )
    }
}
