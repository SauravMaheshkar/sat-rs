use std::io;
use std::io::Read;

use notation::problem::Problem;

mod notation;
mod parser;

use parser::cnfparser;

fn main() {
    // SAT Solver written in Rust
    // Usage: cat problem.cnf | ./sat

    // Read from stdin
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    // Print the contents of the file
    println!("{}", buffer);

    let _prob: Problem = cnfparser::parse_cnf(&buffer);

    // Parse the problem
    // let _problem = parse(&problem);
}
