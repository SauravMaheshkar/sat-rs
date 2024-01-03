//! Collection of structs representing the notation used in the SAT solver
//!
//! * [`Literal`] - A struct representing a literal (atom)
//! * [`Clause`] - A struct representing a clause
//! * [`Formula`] - A struct representing a propositional formula
use std::collections::HashMap;

/// Struct representing a Literal. Also known as an atom.
///
/// Derives from [`Debug`] and [`Clone`].
/// Contains a value and a boolean representing whether it is negated or not
///
/// # Examples
/// ```rust
/// use sat_rs::notation::Literal;
///
/// let literal = Literal::new();
///
/// let literal = Literal::from_value(1);
///
/// let literal = Literal{ value: 1, negated: false};
/// ```
#[derive(Debug, Clone)]
pub struct Literal {
    pub value: i32,
    pub negated: bool,
}

impl Literal {

    /// Creates a new [`Literal`] with value `0` and negated set to `false`
    ///
    /// # Examples
    /// ```rust
    /// use sat_rs::notation::Literal;
    ///
    /// let literal = Literal::new();
    /// ```
    #[allow(dead_code)]
    pub fn new() -> Literal {
        return Literal {
            value: 0,
            negated: false,
        }
    }

    /// Creates a new [`Literal`] from a given value. Negated is set to `false`
    ///
    /// # Arguments
    /// * `value` - An [`i32`] representing the value of the [`Literal`]
    ///
    /// # Examples
    /// ```rust
    /// use sat_rs::notation::Literal;
    ///
    /// let literal = Literal::from_value(1);
    /// ```
    #[allow(dead_code)]
    pub fn from_value(value: i32) -> Literal {
        return Literal {
            value: value,
            negated: false,
        }
    }
}

/// Struct representing a Clause
///
/// Derives from [`Debug`] and [`Clone`].
///
/// Contains a vector of [`Literal`]s
///
/// # Examples
/// ```rust
/// use sat_rs::notation::Literal;
/// use sat_rs::notation::Clause;
///
/// let mut clause = Clause::new();
///
/// let p = Literal{ value: 1, negated: false};
/// let q = Literal{ value: 2, negated: false};
///
/// clause.literals.push(p);
/// clause.literals.push(q);
/// ```
#[derive(Debug, Clone)]
pub struct Clause {
    pub literals: Vec<Literal>,
}

impl Clause {

    /// Creates a new [`Clause`] with an empty vector of [`Literal`]s
    ///
    /// # Examples
    /// ```rust
    /// use sat_rs::notation::Clause;
    ///
    /// let clause = Clause::new();
    /// ```
    pub fn new() -> Clause {
        Clause {
            literals: Vec::new(),
        }
    }
}

/// Struct representing a Propositional Formula
///
/// Derives from [`Debug`] and [`Clone`].
///
/// Contains a vector of [`Clause`]s, a vector of [`Literal`]s, the number of clauses and the number of variables
#[derive(Debug, Clone)]
pub struct Formula {
    pub clauses: Vec<Clause>,
    pub literals: Vec<i32>,
    pub num_clauses: i32,
    pub num_vars: i32,
}


impl Formula {

    /// Creates a new [`Formula`] with an empty vector of [`Clause`]s and [`Literal`]s
    ///
    /// # Examples
    /// ```rust
    /// use sat_rs::notation::Formula;
    ///
    /// let formula = Formula::new();
    /// ```
    #[allow(dead_code)]
    pub fn new() -> Formula {
        Formula {
            clauses: Vec::new(),
            literals: Vec::new(),
            num_clauses: 0,
            num_vars: 0,
        }
    }

    #[allow(dead_code)]
    /// Evaluates a [`Formula`] given an interpretation
    ///
    /// # Arguments
    /// * `interpretation` - A [`HashMap`] of [`i32`] and [`bool`] representing the interpretation
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
    pub fn evaluate(&mut self, interpretation: &HashMap<i32, bool>) -> bool {
        let mut value: bool = true;
        for clause in &mut self.clauses {
            let mut clause_value: bool = false;
            for literal in &mut clause.literals {
                let mut literal_value: bool = false;
                if interpretation.contains_key(&literal.value) {
                    literal_value = interpretation[&literal.value];
                }
                if literal.negated {
                    literal_value = !literal_value;
                }
                if literal_value {
                    clause_value = true;
                    break;
                }
            }
            value = value && clause_value;
        }
        value
    }
}
