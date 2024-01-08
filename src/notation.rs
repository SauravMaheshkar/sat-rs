//! Collection of structs representing the notation used in the SAT solver
//!
//! * [`Literal`] - A struct representing a literal (atom)
//! * [`Clause`] - A struct representing a clause
//! * [`Formula`] - A struct representing a propositional formula
use std::collections::HashMap;

/// Struct representing a Literal.
///
/// Derives from [`Debug`], [`Clone`] and [`PartialEq`].
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
#[derive(Debug, Clone, PartialEq)]
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
        };
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
            value,
            negated: false,
        };
    }

    /// Evaluates a [`Literal`] given a boolean value
    ///
    /// # Arguments
    /// * `given_value` - A [`bool`] representing the value of the [`Literal`]
    ///
    /// # Examples
    /// ```rust
    /// use sat_rs::notation::Literal;
    ///
    /// let mut literal = Literal{ value: 1, negated: false};
    /// assert_eq!(literal.evaluate(true), true);
    ///
    /// let mut literal = Literal{ value: 1, negated: true};
    /// assert_eq!(literal.evaluate(true), false);
    /// ```
    pub fn evaluate(&mut self, given_value: bool) -> bool {
        if self.negated {
            return !given_value;
        } else {
            return given_value;
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
    pub is_satisfied: bool,
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
            is_satisfied: false,
        }
    }

    /// Evaluates a [`Clause`] given an interpretation
    ///
    /// # Arguments
    /// * `interpretation` - A [`HashMap`] of [`i32`] and [`bool`] representing the interpretation
    ///
    /// # Returns
    /// * `bool` - Returns `true` if the clause is true in the `interpretation`, otherwise `false`
    ///
    /// # Examples
    /// ```rust
    /// use sat_rs::notation::{Literal, Clause};
    /// use std::collections::HashMap;
    ///
    /// let mut clause = Clause::new();
    /// let p = Literal{ value: 1, negated: false};
    /// clause.literals.push(p); // Clause: p
    ///
    /// let mut interpretation: HashMap<i32, bool> = HashMap::new();
    /// interpretation.insert(1, true);
    ///
    /// assert_eq!(clause.evaluate(&interpretation), true);
    ///
    /// let mut clause = Clause::new();
    /// let p = Literal{ value: 1, negated: true};
    /// let q = Literal{ value: 1, negated: false};
    /// clause.literals.push(p);
    /// clause.literals.push(q); // Clause: -p v p
    ///
    /// assert_eq!(clause.evaluate(&interpretation), false);
    /// ```
    pub fn evaluate(&mut self, interpretation: &HashMap<i32, bool>) -> bool {
        // TODO(SauravMaheshkar): Remove monkeypatch
        let mut temp = None;
        let mut clausal_value: bool = false;

        for literal in &mut self.literals {
            // Disjunction of Literals
            if temp == None {
                clausal_value = literal.evaluate(interpretation[&literal.value]);
                temp = Some(clausal_value);
            } else if temp == Some(true) {
                clausal_value = clausal_value || literal.evaluate(interpretation[&literal.value]);
            }
        }

        // If clause is satisfied, set is_satisfied to true
        if clausal_value {
            self.is_satisfied = true;
        }

        return clausal_value;
    }
}

/// Struct representing a Propositional Formula
///
/// Derives from [`Debug`] and [`Clone`].
///
/// Contains a vector of [`Clause`]s, a vector of [`Literal`]s, a vector of variables, the number of clauses and the number of variables
#[derive(Debug, Clone)]
pub struct Formula {
    pub clauses: Vec<Clause>,
    pub literals: Vec<Literal>,
    pub vars: Vec<i32>,
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
            vars: Vec::new(),
            num_clauses: 0,
            num_vars: 0,
        }
    }

    /// Evaluates a [`Formula`] given an interpretation
    ///
    /// # Arguments
    /// * `interpretation` - A [`HashMap`] of [`i32`] and [`bool`] representing the interpretation
    ///
    /// # Returns
    /// * `bool` - Returns `true` if the formula is true in the `interpretation`, otherwise `false`
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
    /// use sat_rs::notation::{Literal, Clause, Formula};
    /// use std::collections::HashMap;
    ///
    /// let mut clause = Clause::new();
    /// let p = Literal{ value: 1, negated: false};
    /// clause.literals.push(p);
    ///
    /// let mut formula = Formula {
    ///   clauses: vec![clause],
    ///   literals: vec![Literal{ value: 1, negated: false}],
    ///   vars: vec![1],
    ///   num_clauses: 1,
    ///   num_vars: 1,
    /// };
    ///
    /// let mut interpretation: HashMap<i32, bool> = HashMap::new();
    /// interpretation.insert(1, true);
    ///
    /// assert_eq!(formula.evaluate(&interpretation), true);
    /// ```
    pub fn evaluate(&mut self, interpretation: &HashMap<i32, bool>) -> bool {
        // TODO(SauravMaheshkar): Remove monkeypatch
        let mut temp = None;
        let mut value = false;

        // Evaluate the formula
        for clause in &mut self.clauses {
            // Conjunction of Clauses
            if temp == None {
                value = clause.evaluate(interpretation);
                temp = Some(value);
            } else if temp == Some(true) {
                value = value && clause.evaluate(interpretation);
            }
        }
        return value;
    }
}
