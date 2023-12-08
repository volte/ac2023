use std::collections::HashMap;

use crate::{aoc::prelude::*, util::scanner::Scanner};

pub struct SolutionPart1;
pub struct SolutionPart2;

pub const SOLUTIONS: Solutions = (&SolutionPart1, &SolutionPart2);

#[derive(Debug, Clone)]
struct Node {
    left: String,
    right: String,
}

#[derive(Debug, Clone)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Map {
    path: Vec<Direction>,
    nodes: HashMap<String, Node>,
}

impl Map {
    pub fn parse(input: &str) -> Map {
        let mut scanner = Scanner::new(input, true);
        let path_str = scanner.scan_regex("[LR]+").as_str().to_string();

        let path = path_str
            .chars()
            .map(|c| match c {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => panic!("Invalid direction: {}", c),
            })
            .collect::<Vec<_>>();

        let mut nodes = HashMap::new();
        while !scanner.is_finished() {
            let name = scanner.scan_regex("[A-Z0-9]{3}").as_str().to_string();
            scanner.scan_string("= (");
            let left = scanner.scan_regex("[A-Z0-9]{3}").as_str().to_string();
            scanner.scan_string(",");
            let right = scanner.scan_regex("[A-Z0-9]{3}").as_str().to_string();
            scanner.scan_string(")");
            nodes.insert(name, Node { left, right });
        }
        Map { path, nodes }
    }

    pub fn path_length<S: Into<String>, P>(&self, start: S, target: P) -> usize
    where
        P: Fn(&String) -> bool,
    {
        let mut current = start.into();
        for (i, direction) in self.path.iter().cycle().enumerate() {
            let node = self.nodes.get(&current).unwrap();
            current = match direction {
                Direction::Left => node.left.clone(),
                Direction::Right => node.right.clone(),
            };
            if target(&current) {
                return i + 1;
            }
        }
        unreachable!();
    }
}

impl Aoc for SolutionPart1 {
    fn solve(&self, input: &str) -> String {
        let map = Map::parse(input);
        map.path_length("AAA", |x| x == "ZZZ").to_string()
    }
}

impl Aoc for SolutionPart2 {
    fn solve(&self, input: &str) -> String {
        let map = Map::parse(input);
        map.nodes
            .keys()
            .filter(|x| x.ends_with("A"))
            .map(|x| map.path_length(x, |x| x.ends_with("Z")))
            .reduce(num::integer::lcm)
            .unwrap_or(0)
            .to_string()
    }
}
