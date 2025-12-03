use std::{collections::HashSet, fs};

#[derive(Debug)]
struct Range {
    low: i64,
    high: i64,
}

impl From<&str> for Range {
    fn from(item: &str) -> Self {
        let (s1, s2) = item.split_once("-").unwrap();
        let low = i64::from_str_radix(s1, 10).unwrap();
        let high = i64::from_str_radix(s2, 10).unwrap();
        Range { low, high }
    }
}

fn highest_previous(x: i64, divisor: u32) -> i64 {
    let mut xmut = x;
    let s = x.to_string();
    let frac_len = s.len() as u32 / divisor;
    let frac_pow = i64::pow(10, frac_len);
    if s.len() as u32 % divisor != 0 {
        return frac_pow - 1;
    } else {
        let mut trills: Vec<i64> = Vec::new();
        for _ in 0..divisor {
            trills.push(xmut % frac_pow);
            xmut = xmut / frac_pow;
        }

        let tmax = trills.len() - 1;
        let left = trills[tmax];
        for i in 0..tmax {
            if trills[tmax - i] > trills[tmax - (i + 1)] {
                return left - 1;
            } else if trills[tmax - i] < trills[tmax - (i + 1)] {
                return left;
            }
        }
        return left;
    }
}

fn lowest_next(x: i64, divisor: u32) -> i64 {
    let mut xmut = x;
    let s = x.to_string();
    let frac_len = s.len() as u32 / divisor;
    let frac_pow = i64::pow(10, frac_len);
    if s.len() as u32 % divisor != 0 {
        return frac_pow;
    } else {
        let mut trills: Vec<i64> = Vec::new();
        for _ in 0..divisor {
            trills.push(xmut % frac_pow);
            xmut = xmut / frac_pow;
        }
        let tmax = trills.len() - 1;
        let left = trills[tmax];
        for i in 0..tmax {
            if trills[tmax - i] < trills[tmax - (i + 1)] {
                return left + 1;
            } else if trills[tmax - i] > trills[tmax - (i + 1)] {
                return left;
            }
        }
        return left;
    }
}

fn to_vendor_id(x: i64, divisor: u32) -> i64 {
    let s = x.to_string();
    let len = s.len() as u32;
    let pow = i64::pow(10, len);
    let mut vendor_id = 0;
    for i in 0..divisor {
        vendor_id += x * pow.pow(i);
    }
    vendor_id
}

impl Range {
    fn max_divisor(&self) -> u32 {
        self.high.to_string().len() as u32
    }
    fn highest_previous(&self, divisor: u32) -> i64 {
        highest_previous(self.high, divisor)
    }

    fn lowest_next(&self, divisor: u32) -> i64 {
        lowest_next(self.low, divisor)
    }

    fn accumulate_by_divisor(&self, divisor: u32) -> HashSet<i64> {
        let low = self.lowest_next(divisor);
        let high = self.highest_previous(divisor);
        let mut set = HashSet::new();
        for x in low..high + 1 {
            set.insert(to_vendor_id(x, divisor));
        }
        set
    }

    fn accumulate(&self) -> i64 {
        let mut set = HashSet::new();
        for divisor in 2..self.max_divisor() + 1 {
            set.extend(self.accumulate_by_divisor(divisor))
        }
        set.into_iter().sum()
    }
}

fn parse_input(path: &str) -> Vec<Range> {
    fs::read_to_string(path)
        .unwrap()
        .split(",")
        .map(|x| x.into())
        .collect()
}

pub fn star03() {
    let ranges = parse_input("inputs/input02.txt");
    let n: i64 = ranges
        .iter()
        .map(|x| x.accumulate_by_divisor(2).into_iter().sum::<i64>())
        .sum();
    println!("{n}");
}

pub fn star04() {
    let ranges: Vec<Range> = parse_input("inputs/input02.txt");
    let n: i64 = ranges.iter().map(|x| x.accumulate()).sum();
    println!("{n}");
}
