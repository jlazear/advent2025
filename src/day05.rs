use std::{cmp, fs};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
struct Range {
    low: i64,
    high: i64,
}

#[derive(Debug)]
struct Ranges {
    ranges: Vec<Range>,
}

impl Range {
    fn from_str(s: &str) -> Self {
        let split = s.split_once("-").unwrap();
        let low = split.0.parse().unwrap();
        let high = split.1.parse().unwrap();
        Self { low, high }
    }

    fn merge(&self, other: &Range) -> Option<Range> {
        if self.low > other.high || other.low > self.high {
            return None;
        }
        let low = cmp::min(self.low, other.low);
        let high = cmp::max(self.high, other.high);
        Some(Range { low, high })
    }
}

impl Ranges {
    fn has(&self, ingredient: &i64) -> bool {
        for range in &self.ranges {
            if &range.low <= ingredient && ingredient <= &range.high {
                return true;
            }
        }
        return false;
    }

    fn sort(&mut self) {
        self.ranges.sort()
    }

    fn reduce(&mut self) {
        self.sort();
        let mut new_ranges = Vec::new();
        for range in self.ranges.iter() {
            if new_ranges.len() == 0 {
                new_ranges.push(*range);
            } else {
                let last = new_ranges.last_mut().unwrap();
                if let Some(merged) = range.merge(&last) {
                    *last = merged;
                } else {
                    new_ranges.push(*range);
                }
            }
        }
        self.ranges = new_ranges;
    }

    fn count(&self) -> i64 {
        self.ranges
            .iter()
            .map(|range| range.high - range.low + 1)
            .sum()
    }
}

fn parse_input(path: &str) -> (Ranges, Vec<i64>) {
    let s = fs::read_to_string(path).unwrap();
    let (ranges_str, ingredients_str) = s.split_once("\n\n").unwrap();
    let ranges = ranges_str
        .split("\n")
        .map(|line| Range::from_str(line))
        .collect();

    let ingredients = ingredients_str
        .split("\n")
        .map(|x| x.parse().unwrap())
        .collect();

    (Ranges { ranges }, ingredients)
}

pub fn star09() {
    let (ranges, ingredients) = parse_input("inputs/input05.txt");

    let n_fresh: u64 = ingredients
        .iter()
        .map(|ingredient| ranges.has(ingredient) as u64)
        .sum();
    println!("{n_fresh}");
}

pub fn star10() {
    let (mut ranges, _) = parse_input("inputs/input05.txt");
    ranges.reduce();
    println!("{}", ranges.count());
}
