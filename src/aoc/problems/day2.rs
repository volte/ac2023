use crate::{aoc::prelude::*, util::scanner::Scanner};

pub struct SolutionPart1;
pub struct SolutionPart2;

pub const SOLUTIONS: Solutions = (&SolutionPart1, &SolutionPart2);

#[derive(Debug, Default, Clone)]
struct GameHand {
    red: i32,
    green: i32,
    blue: i32,
}

impl GameHand {
    fn new(red: i32, green: i32, blue: i32) -> GameHand {
        GameHand { red, green, blue }
    }

    fn max(&self, other: &GameHand) -> GameHand {
        GameHand::new(
            self.red.max(other.red),
            self.green.max(other.green),
            self.blue.max(other.blue),
        )
    }
}

#[derive(Debug, Default, Clone)]
struct Game {
    id: i32,
    hands: Vec<GameHand>,
}

struct GameIterator<'s> {
    string: &'s str,
}

impl GameIterator<'_> {
    fn new(string: &str) -> GameIterator {
        GameIterator { string }
    }
}

impl Iterator for GameIterator<'_> {
    type Item = Game;

    fn next(&mut self) -> Option<Self::Item> {
        if self.string.is_empty() {
            return None;
        }

        let mut game = Game {
            id: 0,
            hands: Vec::new(),
        };

        let mut scanner = Scanner::new(self.string, true);
        scanner.try_scan_string("Game ").unwrap();
        game.id = scanner.try_scan_int().unwrap();
        scanner.try_scan_string(":").unwrap();

        loop {
            let mut hand = GameHand::default();
            loop {
                let count = scanner.try_scan_int().unwrap();
                let color: &str = scanner.try_scan_regex(r"(red|green|blue)").unwrap();
                let color = match color {
                    "red" => &mut hand.red,
                    "green" => &mut hand.green,
                    "blue" => &mut hand.blue,
                    _ => panic!("Invalid color"),
                };
                *color += count;

                if scanner.try_scan_string(",").is_none() {
                    break;
                }
            }
            game.hands.push(hand);

            if scanner.try_scan_string(";").is_none() {
                break;
            }
        }

        self.string = scanner.string;
        Some(game)
    }
}

impl Aoc for SolutionPart1 {
    fn solve(&self, input: &str) -> String {
        const MAX_RED: i32 = 12;
        const MAX_GREEN: i32 = 13;
        const MAX_BLUE: i32 = 14;

        let result: i32 = GameIterator::new(input)
            .filter(|game| {
                game.hands.iter().all(|hand| {
                    hand.red <= MAX_RED && hand.green <= MAX_GREEN && hand.blue <= MAX_BLUE
                })
            })
            .map(|game| game.id)
            .sum();

        result.to_string()
    }
}

impl Aoc for SolutionPart2 {
    fn solve(&self, input: &str) -> String {
        let result: i32 = GameIterator::new(input)
            .map(|game| {
                let max_hand = game
                    .hands
                    .iter()
                    .fold(GameHand::default(), |acc, hand| acc.max(hand));
                max_hand.red * max_hand.green * max_hand.blue
            })
            .sum();

        result.to_string()
    }
}
