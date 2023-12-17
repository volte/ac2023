use crate::aoc::prelude::*;

pub const SOLUTIONS: Solutions = (&SolutionPart1, &SolutionPart2);

pub struct SolutionPart1;
pub struct SolutionPart2;

impl Aoc for SolutionPart1 {
    fn solve(&self, input: &str) -> String {
        let mut sum = 0;
        for line in input.lines() {
            let first_digit_index = line.find(|p: char| p.is_digit(10)).unwrap();
            let second_digit_index = line.rfind(|p: char| p.is_digit(10)).unwrap();
            let first_digit = line.chars().nth(first_digit_index).unwrap();
            let second_digit = line.chars().nth(second_digit_index).unwrap();
            let number =
                10 * first_digit.to_digit(10).unwrap() + second_digit.to_digit(10).unwrap();

            sum += number;
        }
        sum.to_string()
    }
}

struct DigitScanner<'s> {
    string: &'s str,
}

impl DigitScanner<'_> {
    fn new(string: &str) -> DigitScanner {
        DigitScanner { string }
    }

    fn scan(&mut self) -> Option<u8> {
        if self.string.is_empty() {
            return None;
        }

        let string = self.string;
        self.string = &self.string[1..];

        if let Some(digit) = string.chars().next().unwrap().to_digit(10) {
            return Some(digit as u8);
        }

        for (idx, word) in [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ]
        .iter()
        .enumerate()
        {
            if string.starts_with(word) {
                return Some(1 + idx as u8);
            }
        }

        None
    }

    fn scan_next_digit(&mut self) -> Option<u8> {
        loop {
            if let Some(digit) = self.scan() {
                return Some(digit);
            }
            if self.string.is_empty() {
                return None;
            }
        }
    }
}

impl Aoc for SolutionPart2 {
    fn solve(&self, input: &str) -> String {
        let mut sum = 0;
        for line in input.lines() {
            let mut scanner = DigitScanner::new(line);
            let first_digit = scanner.scan_next_digit().unwrap();
            let mut last_digit = first_digit;
            while let Some(digit) = scanner.scan_next_digit() {
                last_digit = digit;
            }
            let number = 10 * first_digit as u32 + last_digit as u32;
            sum += number;
        }
        sum.to_string()
    }
}
