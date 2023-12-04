use std::collections::HashSet;

use crate::{aoc::prelude::*, util::scanner::Scanner};

pub struct SolutionPart1;
pub struct SolutionPart2;

pub const SOLUTIONS: Solutions = (&SolutionPart1, &SolutionPart2);

#[derive(Default, Debug, Clone)]
struct Card {
    id: i32,
    winning_numbers: Vec<i32>,
    my_numbers: Vec<i32>,
}

impl Card {
    fn win_count(&self) -> i32 {
        self.winning_numbers
            .iter()
            .filter(|&n| self.my_numbers.contains(n))
            .count() as i32
    }
}

struct CardIterator<'s> {
    scanner: Scanner<'s>,
}

impl CardIterator<'_> {
    fn new(string: &str) -> CardIterator {
        CardIterator {
            scanner: Scanner::new(string, true),
        }
    }
}

impl Iterator for CardIterator<'_> {
    type Item = Card;

    fn next(&mut self) -> Option<Self::Item> {
        if self.scanner.is_finished() {
            return None;
        }

        let mut card = Card::default();

        self.scanner.scan_string("Card");
        card.id = self.scanner.scan_signed_int().parse::<i32>();
        self.scanner.scan_string(":");

        loop {
            match self.scanner.try_scan_unsigned_int() {
                Some(token) => card.winning_numbers.push(token.parse::<i32>()),
                None => break,
            }
        }

        self.scanner.scan_string("|");

        loop {
            match self.scanner.try_scan_unsigned_int() {
                Some(token) => card.my_numbers.push(token.parse::<i32>()),
                None => break,
            }
        }

        Some(card)
    }
}

impl Aoc for SolutionPart1 {
    fn solve(&self, input: &str) -> String {
        CardIterator::new(input)
            .map(|card| i32::pow(2, card.win_count() as u32) as i32 / 2)
            .sum::<i32>()
            .to_string()
    }
}

#[derive(Debug, Clone)]
struct CardCounter {
    counts: Vec<u32>,
}

impl CardCounter {
    fn new(capacity: usize) -> CardCounter {
        CardCounter {
            counts: vec![1; capacity],
        }
    }

    fn update(&mut self, card: &Card) {
        let idx = card.id - 1;
        let num_copies = self.counts[idx as usize];
        let win_count = card.win_count();
        for i in (idx + 1)..=(idx + win_count) {
            self.counts.get_mut(i as usize).map(|c| *c += num_copies);
        }
    }

    fn total_count(&self) -> u32 {
        self.counts.iter().sum()
    }
}

impl Aoc for SolutionPart2 {
    fn solve(&self, input: &str) -> String {
        let cards = CardIterator::new(input).collect::<Vec<_>>();
        let mut counter = CardCounter::new(cards.len());

        for card in cards.iter() {
            counter.update(card);
        }

        counter.total_count().to_string()
    }
}
