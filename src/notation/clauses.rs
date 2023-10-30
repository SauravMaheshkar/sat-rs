use std::fmt::Debug;

use notation::literals::Literal;

pub struct Clause {
    literals: Vec<Literal>,
}

impl Clause {
    pub fn new(literals: Vec<Literal>) -> Clause {
        Clause { literals: literals }
    }
}

impl Debug for Clause {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Clause {{ literals: {:?} }}", self.literals)
    }
}
