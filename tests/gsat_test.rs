use sat_rs::cnfparser;
use sat_rs::solvers::gsat;

#[test]
fn test_gsat() {
    // Create a buffer of type &str using file at bin/unsatisfiable.cnf
    let buffer = include_str!("../bin/unsatisfiable.cnf");

    // Parse the CNF file
    let formula = cnfparser::parse_cnf(&buffer);

    // Check that the formula is unsatisfiable
    let result: bool = gsat::gsat_algorithm(&mut formula.unwrap(), 10, 10);

    assert_eq!(result, false);
}
