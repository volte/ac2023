use crate::{aoc::prelude::*, util::scanner::Scanner};

pub struct SolutionPart1;
pub struct SolutionPart2;

pub const SOLUTIONS: Solutions = (&SolutionPart1, &SolutionPart2);

#[derive(Default, Debug, Clone, Copy)]
struct Race {
    length: i64,
    record: i64,
}

struct RaceIterator<'s> {
    time_scanner: Scanner<'s>,
    distance_scanner: Scanner<'s>,
}

impl RaceIterator<'_> {
    fn new(input: &str) -> RaceIterator {
        let lines = input.lines().take(2).collect::<Vec<_>>();

        let mut time_scanner = Scanner::new(lines[0], true);
        let mut distance_scanner = Scanner::new(lines[1], true);

        time_scanner.scan_string("Time:");
        distance_scanner.scan_string("Distance:");

        RaceIterator {
            time_scanner,
            distance_scanner,
        }
    }
}

impl Iterator for RaceIterator<'_> {
    type Item = Race;

    fn next(&mut self) -> Option<Self::Item> {
        if self.time_scanner.is_finished() {
            return None;
        }
        let time = self.time_scanner.scan_unsigned_int().parse::<i64>();
        let distance = self.distance_scanner.scan_unsigned_int().parse::<i64>();
        return Some(Race {
            length: time,
            record: distance,
        });
    }
}

// h        :  time holding button (= speed)
// L        :  length of race
// R        :  record
// distance = h * (L - h)
//
// Solve:
//        h * (L - h)   >   R
//      -h^2 + Lh - R   >   0
//
//  (-L ± sqrt(L^2 - 4R)) / 2    <   h    <    (-L ∓ sqrt(L^2 - 4R))

impl Aoc for SolutionPart1 {
    fn solve(&self, input: &str) -> String {
        RaceIterator::new(input)
            .map(|race| {
                let l = race.length as f64;

                // Add a small epsilon because we need to beat the record, not tie it, and
                // the solutions of the unperturbed quadratic equation would only tie it.
                let r = race.record as f64 + 1e-3;

                let t1 = -(-l + f64::sqrt(l * l - 4.0 * r)) / 2.0;
                let t2 = -(-l - f64::sqrt(l * l - 4.0 * r)) / 2.0;

                let min_time = f64::ceil(f64::min(t1, t2)) as i64;
                let max_time = f64::floor(f64::max(t1, t2)) as i64;

                max_time - min_time + 1
            })
            .product::<i64>()
            .to_string()
    }
}

impl Aoc for SolutionPart2 {
    fn solve(&self, input: &str) -> String {
        let input = input.replace(" ", "");
        SolutionPart1.solve(input.as_str())
    }
}
