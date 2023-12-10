use super::Solutions;

mod day1;
mod day10;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

/// Return a vector of all solved problems, in order as a pair of (part1, part2)
pub fn all_problems() -> Vec<Solutions> {
    vec![
        day1::SOLUTIONS,
        day2::SOLUTIONS,
        day3::SOLUTIONS,
        day4::SOLUTIONS,
        day5::SOLUTIONS,
        day6::SOLUTIONS,
        day7::SOLUTIONS,
        day8::SOLUTIONS,
        day9::SOLUTIONS,
        day10::SOLUTIONS,
    ]
}
