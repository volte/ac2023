use crate::aoc::prelude::*;

pub struct SolutionPart1;
pub struct SolutionPart2;

pub const SOLUTIONS: Solutions = (&SolutionPart1, &SolutionPart2);

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn continue_sequence(seq: &Vec<i64>) -> i64 {
    let diffs = seq
        .into_iter()
        .zip(seq.into_iter().skip(1))
        .map(|(a, b)| b - a)
        .collect::<Vec<_>>();
    let last = seq.last().unwrap();
    last + if diffs.iter().all(|&diff| diff == 0) {
        0
    } else {
        continue_sequence(&diffs)
    }
}

impl Aoc for SolutionPart1 {
    fn solve(&self, input: &str) -> String {
        parse_input(input)
            .iter()
            .map(|seq| continue_sequence(seq))
            .sum::<i64>()
            .to_string()
    }
}

impl Aoc for SolutionPart2 {
    fn solve(&self, input: &str) -> String {
        parse_input(input)
            .iter()
            .map(|seq| continue_sequence(&Iterator::rev(seq.iter().copied()).collect::<Vec<_>>()))
            .sum::<i64>()
            .to_string()
    }
}
