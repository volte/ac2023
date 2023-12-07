use std::marker::PhantomData;

use crate::{aoc::prelude::*, util::scanner::Scanner};

pub struct SolutionPart1;
pub struct SolutionPart2;

pub const SOLUTIONS: Solutions = (&SolutionPart1, &SolutionPart2);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Card(u8);

trait CardSet {
    fn parse_card(input: char) -> Card;
    fn get_hand_type(cards: &[Card]) -> HandType;
}

struct DefaultCardSet;

impl CardSet for DefaultCardSet {
    fn parse_card(input: char) -> Card {
        Card(match input {
            x if x >= '2' && x <= '9' => x as u8 - '0' as u8,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("Invalid card: {}", input),
        })
    }

    fn get_hand_type(cards: &[Card]) -> HandType {
        let mut counts = cards
            .iter()
            .map(|card| cards.iter().filter(|&c| *c == *card).count())
            .collect::<Vec<_>>();
        counts.sort();
        match counts.as_slice() {
            [1, 1, 1, 1, 1] => HandType::HighCard,
            [1, 1, 1, 2, 2] => HandType::OnePair,
            [1, 2, 2, 2, 2] => HandType::TwoPair,
            [1, 1, 3, 3, 3] => HandType::ThreeOfAKind,
            [2, 2, 3, 3, 3] => HandType::FullHouse,
            [1, 4, 4, 4, 4] => HandType::FourOfAKind,
            [5, 5, 5, 5, 5] => HandType::FiveOfAKind,
            _ => panic!("Invalid hand: {:?}", cards),
        }
    }
}

struct JokerCardSet;

impl JokerCardSet {
    const JOKER: Card = Card(0);
}

impl CardSet for JokerCardSet {
    fn parse_card(input: char) -> Card {
        Card(match input {
            'J' => 0,
            x if x >= '2' && x <= '9' => x as u8 - '0' as u8,
            'T' => 10,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("Invalid card: {}", input),
        })
    }

    fn get_hand_type(cards: &[Card]) -> HandType {
        let mut counts = cards
            .iter()
            .map(|card| {
                cards.iter().filter(|&c| *c == *card).count()
                    * (if *card == Self::JOKER { 0 } else { 1 })
            })
            .collect::<Vec<_>>();
        counts.sort();
        match counts.as_slice() {
            [0, 0, 0, 0, 0] => HandType::FiveOfAKind,
            [0, 0, 0, 0, 1] => HandType::FiveOfAKind,
            [0, 0, 0, 1, 1] => HandType::FourOfAKind,
            [0, 0, 0, 2, 2] => HandType::FiveOfAKind,
            [0, 0, 1, 1, 1] => HandType::ThreeOfAKind,
            [0, 0, 1, 2, 2] => HandType::FourOfAKind,
            [0, 0, 3, 3, 3] => HandType::FiveOfAKind,
            [0, 1, 1, 1, 1] => HandType::OnePair,
            [0, 1, 1, 2, 2] => HandType::ThreeOfAKind,
            [0, 1, 3, 3, 3] => HandType::FourOfAKind,
            [0, 2, 2, 2, 2] => HandType::FullHouse,
            [0, 4, 4, 4, 4] => HandType::FiveOfAKind,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            [1, 1, 1, 2, 2] => HandType::OnePair,
            [1, 1, 3, 3, 3] => HandType::ThreeOfAKind,
            [1, 2, 2, 2, 2] => HandType::TwoPair,
            [1, 4, 4, 4, 4] => HandType::FourOfAKind,
            [2, 2, 3, 3, 3] => HandType::FullHouse,
            [5, 5, 5, 5, 5] => HandType::FiveOfAKind,
            _ => panic!("Invalid hand: {:?}", cards),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
    bid: i32,
    hand_type: HandType,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand_type
            .cmp(&other.hand_type)
            .then_with(|| self.cards.cmp(&other.cards))
    }
}

#[derive(PartialEq, Eq, Ord, PartialOrd, Debug, Clone, Copy)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

struct HandIterator<'s, C: CardSet> {
    scanner: Scanner<'s>,
    _marker: PhantomData<C>,
}

impl<'s, C: CardSet> HandIterator<'s, C> {
    fn new(input: &'s str) -> Self {
        Self {
            scanner: Scanner::new(input, true),
            _marker: PhantomData,
        }
    }
}

impl<'s, C: CardSet> Iterator for HandIterator<'s, C> {
    type Item = Hand;

    fn next(&mut self) -> Option<Self::Item> {
        if self.scanner.is_finished() {
            return None;
        }
        let cards = (0..5)
            .map(|_| {
                C::parse_card(
                    self.scanner
                        .scan_regex("[2-9TJQKA]")
                        .as_str()
                        .chars()
                        .next()
                        .unwrap(),
                )
            })
            .collect::<Vec<_>>();
        let bid = self.scanner.scan_unsigned_int().parse::<i32>();
        let hand_type = C::get_hand_type(cards.as_slice());
        Some(Hand {
            cards,
            bid,
            hand_type,
        })
    }
}

struct Solution<C: CardSet> {
    _marker: PhantomData<C>,
}

impl<C: CardSet> Default for Solution<C> {
    fn default() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<C: CardSet> Aoc for Solution<C> {
    fn solve(&self, input: &str) -> String {
        let mut hands = HandIterator::<C>::new(input).collect::<Vec<_>>();
        hands.sort();

        hands
            .iter()
            .enumerate()
            .map(|(i, hand)| ((i + 1) as i32) * hand.bid)
            .sum::<i32>()
            .to_string()
    }
}

impl Aoc for SolutionPart1 {
    fn solve(&self, input: &str) -> String {
        Solution::<DefaultCardSet>::default().solve(input)
    }
}

impl Aoc for SolutionPart2 {
    fn solve(&self, input: &str) -> String {
        Solution::<JokerCardSet>::default().solve(input)
    }
}
