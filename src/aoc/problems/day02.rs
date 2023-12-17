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
    scanner: Scanner<'s>,
}

impl GameIterator<'_> {
    fn new(string: &str) -> GameIterator {
        GameIterator {
            scanner: Scanner::new(string, true),
        }
    }
}

impl Iterator for GameIterator<'_> {
    type Item = Game;

    fn next(&mut self) -> Option<Self::Item> {
        if self.scanner.is_finished() {
            return None;
        }

        let mut game = Game {
            id: 0,
            hands: Vec::new(),
        };

        self.scanner.scan_string("Game");
        game.id = self.scanner.scan_signed_int().parse::<i32>();
        self.scanner.scan_string(":");

        loop {
            let mut hand = GameHand::default();
            loop {
                let count = self.scanner.scan_signed_int().parse::<i32>();
                let color: &str = self.scanner.scan_regex(r"(red|green|blue)").as_str();
                let color = match color {
                    "red" => &mut hand.red,
                    "green" => &mut hand.green,
                    "blue" => &mut hand.blue,
                    _ => panic!("Invalid color"),
                };
                *color += count;

                if self.scanner.try_scan_string(",").is_none() {
                    break;
                }
            }
            game.hands.push(hand);

            if self.scanner.try_scan_string(";").is_none() {
                break;
            }
        }

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
