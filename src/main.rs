extern crate clap;
use clap::Parser;

mod notation;
mod cnfparser;
mod solvers;
mod utils;

use utils::read_file;
use solvers::syntactic::syntactic_algorithm;

#[derive(Parser)]
struct Cli {
    // The path to the CNF file
    path: std::path::PathBuf,

    // which solver to use
    #[clap(short, long, default_value = "syntactic")]
    solver: String,
}

fn main(){
    // SAT Solver written in Rust
    // Usage: ./sat-rs <CNF_FILE> <SOLVER>

    let args = Cli::parse();

    // Check file extension
    let extension = args.path.extension().unwrap_or_default().to_str().unwrap_or_default();
    if extension != "cnf" {
        panic!("File extension must be .cnf");
    }

    let buffer: String = read_file(&args.path);

    // Parse the CNF file
    let formula = cnfparser::parse_cnf(&buffer);

    if args.solver == "syntactic" {
        println!("Using syntactic solver");

        match syntactic_algorithm(formula.unwrap()){
            Ok(value) => println!("Value: {}", value),
            Err(why) => panic!("Error: {}", why),
        };

    } else {
        panic!("Unknown solver: {}", args.solver);
    }
}
