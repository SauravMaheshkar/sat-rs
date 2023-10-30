use notation::literals::Literal;
use notation::problem::Problem;

use notation::clauses::Clause;
use notation::variables::Variable;

pub fn parse_cnf(_buffer: &str) -> Problem {
    let mut num_clauses: u32 = 0;
    let mut num_variables: u32 = 0;
    let mut clauses: Vec<Clause> = Vec::new();

    for line in _buffer.lines() {
        if line.starts_with("c") {
            continue;
        } else if line.starts_with("p") {
            (num_variables, num_clauses) = parse_problem_line(line);
        } else {
            let literals: Vec<Literal> = parse_clause_line(line);
            let clause: Clause = Clause::new(literals);
            clauses.push(clause);
        }
    }

    assert_eq!(clauses.len() as u32, num_clauses);

    let prob = Problem::new(clauses, num_clauses, num_variables);
    println!("{:?}", prob);
    prob
}

fn parse_problem_line(_line: &str) -> (u32, u32) {
    let contents: Vec<&str> = _line.split_whitespace().collect();
    let num_variables: u32 = contents[2].parse().unwrap();
    let num_clauses: u32 = contents[3].parse().unwrap();
    (num_variables, num_clauses)
}

fn parse_clause_line(_line: &str) -> Vec<Literal> {
    let mut clause: Vec<&str> = _line.split_whitespace().collect();
    clause.pop();
    let mut literals: Vec<Literal> = Vec::new();
    for literal in clause {
        let variable: String = literal.to_string();
        let negated: bool = variable.starts_with("-");
        let literal: Literal = Literal::new(Variable::new(variable), negated);
        literals.push(literal);
    }
    literals
}
