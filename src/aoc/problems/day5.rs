use crate::{aoc::prelude::*, util::scanner::Scanner};

pub struct SolutionPart1;
pub struct SolutionPart2;

pub const SOLUTIONS: Solutions = (&SolutionPart1, &SolutionPart2);

#[allow(unused)]
mod key {
    pub const SEED: usize = 0;
    pub const SOIL: usize = 1;
    pub const FERTILIZER: usize = 2;
    pub const WATER: usize = 3;
    pub const LIGHT: usize = 4;
    pub const TEMPERATURE: usize = 5;
    pub const HUMIDITY: usize = 6;
    pub const LOCATION: usize = 7;
}

#[derive(Default, Debug, Clone, Copy)]
struct Interval {
    start: i64,
    end: i64,
}

impl Interval {
    fn new(start: i64, end: i64) -> Interval {
        Interval { start, end }
    }

    fn start_length(start: i64, length: usize) -> Interval {
        Interval {
            start,
            end: start + length as i64 - 1,
        }
    }

    fn contains(&self, v: i64) -> bool {
        self.start <= v && v <= self.end
    }
}

#[derive(Default, Debug, Clone)]
struct MappingSpan {
    interval: Interval,
    offset: i64,
}

#[derive(Default, Debug, Clone)]
struct Mapping {
    spans: Vec<MappingSpan>,
}

impl Mapping {
    fn insert(&mut self, interval: Interval, offset: i64) {
        if let Err(index) = self
            .spans
            .binary_search_by(|span| span.interval.start.cmp(&interval.start))
        {
            self.spans.insert(index, MappingSpan { interval, offset });
        }
    }

    fn map(&self, v: i64) -> i64 {
        let index = self
            .spans
            .binary_search_by(|span| span.interval.start.cmp(&v))
            .map(|index| index as i32)
            .unwrap_or_else(|index| (index as i32) - 1);

        if index < 0 {
            return v;
        }

        let span = &self.spans[index as usize];
        if span.interval.contains(v) {
            return v + span.offset;
        }
        v
    }

    fn map_interval(&self, interval: Interval) -> Vec<Interval> {
        let mut result = Vec::new();
        let mut start = interval.start;

        let Some(mut span_index) = self.spans.iter().position(|span| span.interval.end >= start) else {
            result.push(interval);
            return result;
        };

        while start <= interval.end {
            let Some(span) = self.spans.get(span_index) else {
                break;
            };

            if span.interval.start > start {
                let end = std::cmp::min(span.interval.start - 1, interval.end);
                result.push(Interval::new(start, end));
                start = end + 1;
            } else {
                let end = std::cmp::min(span.interval.end, interval.end);
                result.push(Interval::new(start + span.offset, end + span.offset));
                start = end + 1;
                span_index += 1;
            }
        }

        if start <= interval.end {
            result.push(Interval::new(start, interval.end));
        }

        result
    }
}

#[derive(Default, Debug)]
struct Almanac {
    seeds: Vec<i64>,
    mappings: [Mapping; key::LOCATION + 1],
}

impl Almanac {
    fn parse(input: &str) -> Almanac {
        let mut scanner = Scanner::new(input, true);
        let mut result = Almanac::default();

        scanner.scan_string("seeds:");
        while let Some(seed_token) = scanner.try_scan_unsigned_int() {
            result.seeds.push(seed_token.parse::<i64>());
        }

        for key in 0..key::LOCATION {
            scanner.scan_regex(".* map:");
            while let Some([dest_token, src_token, length_token]) =
                scanner.try_scan_regex_captures(r"(\d+) (\d+) (\d+)")
            {
                let dest = dest_token.parse::<i64>();
                let src = src_token.parse::<i64>();
                let length = length_token.parse::<usize>();
                let offset = dest - src;

                result.mappings[key].insert(Interval::start_length(src, length), offset);
            }
        }

        result
    }

    fn map_seed_to_location(&self, seed: i64) -> i64 {
        let mut x = seed;
        for key in 0..key::LOCATION {
            x = self.mappings[key].map(x);
        }
        x
    }

    fn map_seed_to_location_interval(&self, seeds: Interval) -> Vec<Interval> {
        let mut intervals = vec![seeds];
        for key in 0..key::LOCATION {
            intervals = intervals
                .iter()
                .flat_map(|interval| self.mappings[key].map_interval(*interval))
                .collect::<Vec<_>>();
        }
        intervals
    }
}

impl Aoc for SolutionPart1 {
    fn solve(&self, input: &str) -> String {
        let almanac = Almanac::parse(input);
        almanac
            .seeds
            .iter()
            .map(|&seed| almanac.map_seed_to_location(seed))
            .min()
            .unwrap()
            .to_string()
    }
}

impl Aoc for SolutionPart2 {
    fn solve(&self, input: &str) -> String {
        let almanac = Almanac::parse(input);
        let seed_intervals = almanac
            .seeds
            .chunks(2)
            .map(|chunk| Interval {
                start: chunk[0],
                end: chunk[0] + chunk[1] - 1,
            })
            .collect::<Vec<_>>();
        seed_intervals
            .iter()
            .map(|&interval| {
                almanac
                    .map_seed_to_location_interval(interval)
                    .iter()
                    .map(|i| i.start)
                    .min()
                    .unwrap()
            })
            .min()
            .unwrap()
            .to_string()
    }
}
