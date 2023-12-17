use nalgebra_glm::IVec2;

use crate::aoc::prelude::*;

pub struct SolutionPart1;
pub struct SolutionPart2;

pub const SOLUTIONS: Solutions = (&SolutionPart1, &SolutionPart2);

struct Universe {
    galaxies: Vec<IVec2>,
    width: usize,
    height: usize,
}

impl Universe {
    fn parse(input: &str) -> Self {
        let mut galaxies = Vec::new();
        let mut width = 0;
        let mut height = 0;
        for (y, line) in input.lines().enumerate() {
            height += 1;
            for (x, c) in line.chars().enumerate() {
                width += 1;
                if c == '#' {
                    galaxies.push(IVec2::new(x as i32, y as i32));
                }
            }
        }
        Self {
            galaxies,
            width,
            height,
        }
    }
}

impl Aoc for SolutionPart1 {
    fn solve(&self, input: &str) -> String {
        todo!()
    }
}

impl Aoc for SolutionPart2 {
    fn solve(&self, input: &str) -> String {
        todo!()
    }
}
