use std::fmt::Debug;

pub struct Variable {
    name: String,
}

impl Variable {
    pub fn new(name: String) -> Variable {
        Variable { name: name }
    }
}

impl Debug for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Variable {{ name: {:?} }}", self.name)
    }
}
