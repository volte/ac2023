mod problems;

/// Type alias for a pair of problem solutions
pub type Solutions = (&'static dyn Aoc, &'static dyn Aoc);

/// A solver for an AOC problem
pub trait Aoc {
    /// Solve the problem and return a string representation of the answer
    fn solve(&self, input: &str) -> String;
}

pub fn solve_problem(day: u8, stage: u8, input: &str) -> String {
    let problems = problems::all_problems();
    let &(part1, part2) = problems.get(day as usize - 1).expect("Invalid day");
    match stage {
        1 => part1.solve(input),
        2 => part2.solve(input),
        _ => panic!("Invalid problem stage"),
    }
}

/// Common imports
pub mod prelude {
    pub use crate::aoc::{Aoc, Solutions};
}
