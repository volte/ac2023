use std::collections::{BinaryHeap, HashSet};

use nalgebra_glm::IVec2;

use crate::{aoc::prelude::*, util::scanner::*};

pub struct SolutionPart1;
pub struct SolutionPart2;

pub const SOLUTIONS: Solutions = (&SolutionPart1, &SolutionPart2);

#[derive(Debug, Default, Clone)]
struct Schematic {
    elements: Vec<SchematicElement>,
}

#[derive(Debug, Clone)]
enum SchematicElementType {
    Number(i32),
    Symbol,
}

#[derive(Debug, Clone)]
struct SchematicElement {
    pos: IVec2,
    ty: SchematicElementType,
    adjacent: HashSet<usize>,
}

impl SchematicElement {
    fn new(pos: IVec2, ty: SchematicElementType) -> SchematicElement {
        SchematicElement {
            pos,
            ty,
            adjacent: HashSet::new(),
        }
    }

    fn region(&self) -> (IVec2, IVec2) {
        match self.ty {
            SchematicElementType::Number(n) => (
                self.pos,
                self.pos + IVec2::new(n.to_string().len() as i32 - 1, 0),
            ),
            SchematicElementType::Symbol => (self.pos, self.pos),
        }
    }

    fn is_adjacent(&self, other: &SchematicElement) -> bool {
        let (min_a, max_a) = self.region();
        let (min_b, max_b) = other.region();

        // Two regions are adjacent if expanding the one of them by one unit in any direction
        // results in the other region overlapping with the expanded region
        (min_a - IVec2::new(1, 1) <= max_b && max_a + IVec2::new(1, 1) >= min_b)
            || (min_b - IVec2::new(1, 1) <= max_a && max_b + IVec2::new(1, 1) >= min_a)
    }

    fn number(&self) -> Option<i32> {
        match self.ty {
            SchematicElementType::Number(n) => Some(n),
            _ => None,
        }
    }

    fn is_number(&self) -> bool {
        self.number().is_some()
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
struct SchematicElementHeapKey(IVec2, usize);

impl Ord for SchematicElementHeapKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0
            .y
            .cmp(&other.0.y)
            .then(self.0.x.cmp(&other.0.x))
            .reverse()
    }
}

impl PartialOrd for SchematicElementHeapKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Schematic {
    fn parse(input: &str) -> Schematic {
        let mut result = Schematic::default();
        let mut pos = IVec2::new(0, 0);
        let mut scanner = Scanner::new(input, false);

        let is_past =
            |pos: IVec2, other: IVec2| pos.y > other.y || (pos.y == other.y && pos.x > other.x);

        // Keep track of the schematic elements that might still be adjacent to the current element
        // by storing them in a binary heap sorted by the position of the element's bottom-right corner
        let mut candidates = BinaryHeap::<SchematicElementHeapKey>::new();

        loop {
            if scanner.is_finished() {
                break;
            }

            // Remove elements from the heap that are no longer within range
            while candidates.peek().map_or(false, move |f| is_past(pos, f.0)) {
                candidates.pop();
            }

            // Check for line breaks
            if scanner.try_scan_linebreak().is_some() {
                pos.y += 1;
                pos.x = 0;
                continue;
            }

            if let Some(dots_token) = scanner.try_scan_regex(r"\.+") {
                pos.x += dots_token.len() as i32;
                continue;
            }

            let element_pos = pos;
            let element_type = {
                if let Some(number_token) = scanner.try_scan_unsigned_int() {
                    pos.x += number_token.len() as i32;
                    Some(SchematicElementType::Number(number_token.parse::<i32>()))
                } else if let Some(symbol_token) = scanner.try_scan_regex(r"[^0-9.]") {
                    pos.x += symbol_token.len() as i32;
                    Some(SchematicElementType::Symbol)
                } else {
                    None
                }
            };

            if let Some(element_type) = element_type {
                let element_index = result.elements.len();
                let mut element = SchematicElement::new(element_pos, element_type);

                // Figure out which elements are adjacent to the current element
                candidates
                    .iter()
                    .for_each(|&SchematicElementHeapKey(_, i)| {
                        let other = &mut result.elements[i];
                        if (element.is_number() != other.is_number()) && element.is_adjacent(other)
                        {
                            element.adjacent.insert(i);
                            other.adjacent.insert(element_index);
                        }
                    });

                candidates.push(SchematicElementHeapKey(
                    element.region().1 + IVec2::new(1, 1),
                    element_index,
                ));

                result.elements.push(element);
            }
        }
        result
    }
}

impl Aoc for SolutionPart1 {
    fn solve(&self, input: &str) -> String {
        let schematic = Schematic::parse(input);

        schematic
            .elements
            .iter()
            .filter_map(|element| {
                (!element.adjacent.is_empty())
                    .then(|| element.number())
                    .flatten()
            })
            .sum::<i32>()
            .to_string()
    }
}

impl Aoc for SolutionPart2 {
    fn solve(&self, input: &str) -> String {
        let schematic = Schematic::parse(input);

        schematic
            .elements
            .iter()
            .filter_map(|element| {
                (element.adjacent.len() == 2 && !element.is_number()).then(|| {
                    element
                        .adjacent
                        .iter()
                        .map(|&i| schematic.elements[i].number().unwrap())
                        .product::<i32>()
                })
            })
            .sum::<i32>()
            .to_string()
    }
}
