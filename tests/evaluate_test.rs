use std::collections::HashMap;

use sat_rs::cnfparser;

#[test]
fn test_parse_cnf() {
    // Create a buffer of type &str using file at bin/problem.cnf
    let buffer = include_str!("../bin/problem.cnf");

    // Parse the CNF file
    let formula = cnfparser::parse_cnf(&buffer);

    // Check using a known solution)
    // TODO(SauravMaheshkar): Replace with terminal simulation?
    let mut interpretation: HashMap<i32, bool> = HashMap::new();
    interpretation.insert(1, false);
    interpretation.insert(2, false);
    interpretation.insert(3, false);

    assert_eq!(formula.unwrap().evaluate(&interpretation), false);
}
