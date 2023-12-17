use std::collections::HashMap;

use nalgebra_glm::I64Vec2;

use crate::aoc::prelude::*;

pub struct SolutionPart1;
pub struct SolutionPart2;

pub const SOLUTIONS: Solutions = (&SolutionPart1, &SolutionPart2);

#[derive(Debug, Clone)]
struct Universe {
    galaxies: HashMap<I64Vec2, i64>,
    width: i64,
    height: i64,
}

struct ExpandedUniverse<'u> {
    universe: &'u Universe,
    offsets_x: Vec<i64>,
    offsets_y: Vec<i64>,
}

impl Universe {
    fn parse(input: &str) -> Self {
        let mut galaxies = HashMap::new();

        let width = input.lines().next().unwrap().len() as i64;
        let height = input.lines().count() as i64;

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    galaxies.insert(I64Vec2::new(x as i64, y as i64), galaxies.len() as i64);
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

impl<'u> ExpandedUniverse<'u> {
    fn new(universe: &'u Universe, expansion_factor: i64) -> Self {
        let mut offsets_x = Vec::new();
        let mut offsets_y = Vec::new();

        let mut offset = 0;
        for x in 0..universe.width {
            offsets_x.push(offset);

            let mut is_empty = true;
            for y in 0..universe.height {
                if universe.galaxies.contains_key(&I64Vec2::new(x, y)) {
                    is_empty = false;
                }
            }
            if is_empty {
                offset += expansion_factor - 1;
            }
        }

        offset = 0;
        for y in 0..universe.height {
            offsets_y.push(offset);

            let mut is_empty = true;
            for x in 0..universe.width {
                if universe.galaxies.contains_key(&I64Vec2::new(x, y)) {
                    is_empty = false;
                }
            }
            if is_empty {
                offset += expansion_factor - 1;
            }
        }

        Self {
            universe,
            offsets_x,
            offsets_y,
        }
    }

    fn iter_galaxies(&'u self) -> impl Iterator<Item = (I64Vec2, i64)> + 'u {
        self.universe
            .galaxies
            .iter()
            .map(move |(coord, &idx)| (self.map(coord), idx))
    }

    fn map(&self, coord: &I64Vec2) -> I64Vec2 {
        I64Vec2::new(
            coord.x + self.offsets_x[coord.x as usize],
            coord.y + self.offsets_y[coord.y as usize],
        )
    }
}

struct Solution {
    expansion_factor: i64,
}

impl Solution {
    fn new(expansion_factor: i64) -> Self {
        Self { expansion_factor }
    }
}

impl Aoc for Solution {
    fn solve(&self, input: &str) -> String {
        let universe = Universe::parse(input);
        let expanded_universe = ExpandedUniverse::new(&universe, self.expansion_factor);

        expanded_universe
            .iter_galaxies()
            .flat_map(|(coord_a, idx_a)| {
                expanded_universe
                    .iter_galaxies()
                    .filter_map(move |(coord_b, idx_b)| {
                        (idx_a < idx_b).then(|| {
                            i64::abs(coord_a.x - coord_b.x) + i64::abs(coord_a.y - coord_b.y)
                        })
                    })
            })
            .reduce(i64::wrapping_add)
            .unwrap()
            .to_string()
    }
}

impl Aoc for SolutionPart1 {
    fn solve(&self, input: &str) -> String {
        Solution::new(2).solve(input)
    }
}

impl Aoc for SolutionPart2 {
    fn solve(&self, input: &str) -> String {
        Solution::new(1000000).solve(input)
    }
}
