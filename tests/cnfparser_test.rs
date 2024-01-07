extern crate sat_rs;

use sat_rs::cnfparser;

#[test]
fn test_parse_cnf() {
    // Create a buffer of type &str using file at bin/problem.cnf
    let buffer = include_str!("../bin/problem.cnf");

    // Parse the CNF file
    let formula = cnfparser::parse_cnf(&buffer);

    // Check properties of the formula
    assert_eq!(formula.as_ref().unwrap().num_clauses, 2);
    assert_eq!(formula.as_ref().unwrap().num_vars, 3);
}
