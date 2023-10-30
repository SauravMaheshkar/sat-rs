extern crate sat_rs;
use sat_rs::parser::cnfparser;

#[test]
fn test_parse_cnf() {
    // Create a buffer of type &str using file at bin/problem.cnf
    let buffer = include_str!("../bin/problem.cnf");
    // Parse the buffer
    let _prob = cnfparser::parse_cnf(&buffer);
}
