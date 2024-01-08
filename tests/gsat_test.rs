use sat_rs::cnfparser;
use sat_rs::solvers::gsat;

#[test]
fn test_gsat() {
    // Create a buffer of type &str using file at bin/unsatisfiable.cnf
    let buffer = include_str!("../bin/unsatisfiable.cnf");

    // Parse the CNF file
    let formula = cnfparser::parse_cnf(&buffer);

    // Check that the formula is unsatisfiable
    let result: bool = gsat::gsat_algorithm(&mut formula.clone().unwrap(), 10, 10, None);
    let result_with_walk_probability: bool =
        gsat::gsat_algorithm(&mut formula.clone().unwrap(), 10, 10, Some(0.5));

    assert_eq!(result, false);
    assert_eq!(result_with_walk_probability, false);
}
