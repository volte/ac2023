use std::collections::HashMap;

use crate::{aoc::prelude::*, util::scanner::Scanner};

pub struct SolutionPart1;
pub struct SolutionPart2;

pub const SOLUTIONS: Solutions = (&SolutionPart1, &SolutionPart2);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Spring {
    Good,
    Bad,
    Unknown,
}

impl Spring {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Good,
            '#' => Self::Bad,
            _ => Self::Unknown,
        }
    }
}

struct ConditionRecordIterator<'s> {
    scanner: Scanner<'s>,
}

impl<'s> ConditionRecordIterator<'s> {
    fn new(input: &'s str) -> Self {
        Self {
            scanner: Scanner::new(input, true),
        }
    }
}

impl Iterator for ConditionRecordIterator<'_> {
    type Item = ConditionRecord;

    fn next(&mut self) -> Option<Self::Item> {
        if self.scanner.is_finished() {
            return None;
        }

        let mut springs = Vec::new();
        let mut groups = Vec::new();

        while let Some(token) = self.scanner.try_scan_regex("[?.#]") {
            springs.push(Spring::from_char(token.parse::<char>()));
        }
        while let Some(token) = self.scanner.try_scan_unsigned_int() {
            groups.push(token.parse::<i64>());
            self.scanner.try_scan_string(",");
        }

        Some(ConditionRecord {
            states: springs,
            groups,
        })
    }
}

#[derive(Clone, Debug)]
struct ConditionRecord {
    states: Vec<Spring>,
    groups: Vec<i64>,
}

impl ConditionRecord {
    fn unfold(&self, count: usize) -> Self {
        Self {
            states: std::iter::repeat(self.states.clone())
                .take(count)
                .collect::<Vec<_>>()
                .join(&Spring::Unknown),
            groups: self.groups.repeat(count),
        }
    }

    fn count_arrangements(&self) -> i64 {
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
        struct State<'s> {
            group_prefix: i64,
            head: Option<&'s Spring>,
            tail: &'s [Spring],
            groups: &'s [i64],
        }

        let mut cache = HashMap::<State, i64>::new();

        fn count_arrangements_inner<'s>(
            state: State<'s>,
            cache: &mut HashMap<State<'s>, i64>,
        ) -> i64 {
            if let Some(cached) = cache.get(&state) {
                return *cached;
            }

            let (tail_head, tail_tail) = state
                .tail
                .split_first()
                .map(|(h, t)| (Some(h), t))
                .unwrap_or((None, &[]));
            let result = (|| match state.head {
                None => {
                    if state.group_prefix == 0 && state.groups.is_empty() {
                        return 1;
                    } else if state.groups.len() == 1 && state.groups[0] == state.group_prefix {
                        return 1;
                    } else {
                        return 0;
                    }
                }
                Some(Spring::Good) => {
                    if state.group_prefix > 0 {
                        if state.groups[0] != state.group_prefix {
                            return 0;
                        }
                        return count_arrangements_inner(
                            State {
                                group_prefix: 0,
                                head: tail_head,
                                tail: tail_tail,
                                groups: &state.groups[1..],
                            },
                            cache,
                        );
                    }
                    return count_arrangements_inner(
                        State {
                            group_prefix: 0,
                            head: tail_head,
                            tail: tail_tail,
                            ..state
                        },
                        cache,
                    );
                }
                Some(Spring::Bad) => {
                    if state.group_prefix > 0 {
                        if state.group_prefix >= state.groups[0] {
                            return 0;
                        }
                    } else {
                        if state.groups.is_empty() {
                            return 0;
                        }
                    }
                    return count_arrangements_inner(
                        State {
                            group_prefix: state.group_prefix + 1,
                            head: tail_head,
                            tail: tail_tail,
                            ..state
                        },
                        cache,
                    );
                }
                Some(Spring::Unknown) => {
                    let bad = count_arrangements_inner(
                        State {
                            head: Some(&Spring::Bad),
                            ..state
                        },
                        cache,
                    );
                    let good = count_arrangements_inner(
                        State {
                            head: Some(&Spring::Good),
                            ..state
                        },
                        cache,
                    );
                    return bad + good;
                }
            })();
            cache.insert(state, result);
            result
        }

        let (head, tail) = self.states.split_first().unwrap();

        let result = count_arrangements_inner(
            State {
                group_prefix: 0,
                head: Some(head),
                tail,
                groups: self.groups.as_slice(),
            },
            &mut cache,
        );
        result
    }
}

struct Solution {
    unfold: usize,
}

impl Solution {
    fn new(unfold: usize) -> Self {
        Self { unfold }
    }
}

impl Aoc for Solution {
    fn solve(&self, input: &str) -> String {
        ConditionRecordIterator::new(input)
            .map(|rec| rec.unfold(self.unfold).count_arrangements())
            .sum::<i64>()
            .to_string()
    }
}

impl Aoc for SolutionPart1 {
    fn solve(&self, input: &str) -> String {
        Solution::new(1).solve(input)
    }
}

impl Aoc for SolutionPart2 {
    fn solve(&self, input: &str) -> String {
        Solution::new(5).solve(input)
    }
}
