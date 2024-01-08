# SAT Solvers in Rust

[![built with nix](https://builtwithnix.org/badge.svg)](https://builtwithnix.org)

This crate contains implementations of various satisfiability solvers for the boolean satisfiability problem (SAT).

## Usage
The crate can be used as a library or as a binary. To use it as a binary, run the following command:
```bash
cargo run <CNF_FILE> <SOLVER>
```
OR
```bash
sat-rs <CNF_FILE> <SOLVER>
```

## Algorithms

Available implementations:

* `CHOAS`: A purely random algorithm (appropriately named) which generates random interpretations and checks if the formula evaluates to true.
* `WSAT`: A pseudo-random algorithm which generates random interpretations and if the formula evaluates to false, flips a random variable from some unsatisfied clause and repeats this process until it finds a satisfying implementation.
* `GSAT`: A pseudo-random algorithm which generates random interpretations and if the formula evaluates to false, flips a variable which satisfies the maximum number of unsatisfied clauses and repeats this process until it finds a satisfying implementation.

## References

* Initial work on the crate was based on the [COMP21111: Logic and Modelling](https://portal.manchester.ac.uk/uPortal/p/course-unit-info.ctf1/max/render.uP?pP_action=viewCUDetails&pP_location=/CourseUnitPublishing/CourseUnitDataFiles/COMP/001900COMP211112023-08-011V15.xml) course taught by [Konstantin Korovin](https://www.cs.man.ac.uk/~korovink/).
