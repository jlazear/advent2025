use recursive::recursive;
use std::fs;

fn parse_input(path: &str) -> Vec<Vec<u64>> {
    fs::read_to_string(path)
        .unwrap()
        .split("\n")
        .map(|x| x.as_bytes().into_iter().map(|x| (x - 48) as u64).collect())
        .collect()
}

fn max_joltage(line: Vec<u64>) -> u64 {
    let mut tens = 0;
    let mut ones = 0;
    for x in &line[..line.len() - 1] {
        if *x > tens {
            tens = *x;
            ones = 0;
        } else if *x > ones {
            ones = *x;
        }
    }
    let last = line[line.len() - 1];
    if last > ones {
        ones = last;
    }
    10 * tens + ones
}

fn to_int(line: &[u64]) -> u64 {
    let len = line.len() as u32;
    let mut s = 0;
    for (i, x) in line.iter().enumerate() {
        let exp = len - 1 - (i as u32);
        s += x * u64::pow(10, exp);
    }
    s
}

#[recursive]
fn max_joltage_recursive(line: &[u64], n: usize) -> Vec<u64> {
    let len = line.len();

    if len == n {
        return line.to_vec();
    } else if n == 1 {
        return vec![line.iter().max().unwrap().clone()];
    }
    let n_test = len - n + 1;
    let mut joltage = vec![0];
    for offset in 0..n_test {
        if line[offset] > joltage[0] {
            let newline = &line[offset + 1..];
            joltage.clear();
            joltage.push(line[offset]);
            joltage.extend(max_joltage_recursive(newline, n - 1));
        }
    }

    return joltage;
}

pub fn star05() {
    let lines = parse_input("inputs/input03.txt");
    let total: u64 = lines.into_iter().map(|line| max_joltage(line)).sum();
    println!("{total}");
}

pub fn star06() {
    let lines = parse_input("inputs/input03.txt");
    let total: u64 = lines
        .iter()
        .map(|line| to_int(&max_joltage_recursive(&line, 12)))
        .sum();
    println!("{total}")
}
