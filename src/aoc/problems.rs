use super::Solutions;

mod day1;

/// Return a vector of all solved problems, in order as a pair of (part1, part2)
pub fn all_problems() -> Vec<Solutions> {
    vec![day1::SOLUTIONS]
}
