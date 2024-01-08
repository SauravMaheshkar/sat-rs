use clap::Parser;
use clap_verbosity_flag::Verbosity;

mod cnfparser;
mod notation;
mod solvers;
mod utils;

use solvers::chaos::chaos_algorithm;
use solvers::gsat::gsat_algorithm;
use solvers::interactive::interactive_algorithm;
use solvers::wsat::wsat_algorithm;
use utils::read_file;

#[derive(Parser, Debug)]
struct Cli {
    // The path to the CNF file
    path: std::path::PathBuf,

    // which solver to use
    #[clap(short, long, default_value = "interactive")]
    solver: String,

    // verbosity level
    #[command(flatten)]
    verbosity: Verbosity,
}

fn main() {
    // SAT Solver written in Rust
    // Usage: ./sat-rs <CNF_FILE> <SOLVER>

    let args = Cli::parse();

    // Check file extension
    let extension = args
        .path
        .extension()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default();
    if extension != "cnf" {
        panic!("File extension must be .cnf, got: {}", extension);
    }

    let buffer: String = read_file(&args.path);

    // Parse the CNF file
    let formula = cnfparser::parse_cnf(&buffer);

    let result = match args.solver.as_str() {
        "interactive" => interactive_algorithm(&mut formula.unwrap()),
        "chaos" => chaos_algorithm(&mut formula.unwrap(), 100),
        "wsat" => wsat_algorithm(&mut formula.unwrap(), 100, 100),
        "gsat" => gsat_algorithm(&mut formula.unwrap(), 100, 100),
        &_ => panic!("Unknown solver: {}", args.solver),
    };

    println!("Result: {}", result);
}
