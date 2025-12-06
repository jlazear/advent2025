use std::{fs, iter::zip};

#[derive(Debug)]
enum Operator {
    Multiply,
    Add,
}

impl From<&str> for Operator {
    fn from(item: &str) -> Self {
        match item {
            "*" => Operator::Multiply,
            "+" => Operator::Add,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
struct Problem {
    operands: Vec<i64>,
    operator: Operator,
}

impl Problem {
    fn evaluate(&self) -> i64 {
        match self.operator {
            Operator::Add => self.operands.iter().sum(),
            Operator::Multiply => self.operands.iter().fold(1, |acc, x| acc * x),
        }
    }
}

#[derive(Debug)]
struct Problems {
    problems: Vec<Problem>,
}

#[derive(Debug)]
struct Block {
    block: Vec<Vec<char>>,
}

impl Block {
    fn divide_and_transpose(&self) -> Vec<Vec<i64>> {
        let mut blocks = Vec::new();
        let mut temp_block: Vec<i64> = Vec::new();
        let n_rows = self.block[0].len();
        for col in (0..n_rows).rev() {
            let val_str = self
                .block
                .iter()
                .map(|line| line[col])
                .collect::<String>()
                .trim()
                .to_string();
            if val_str.is_empty() {
                blocks.push(temp_block);
                temp_block = Vec::new();
            } else {
                let val = val_str.parse().unwrap();
                temp_block.push(val);
            }
        }
        blocks.push(temp_block);
        blocks
    }
}

fn parse_input11(path: &str) -> Problems {
    let s = fs::read_to_string(path).unwrap();
    let mut lines: Vec<&str> = s.split("\n").collect();
    let operand_line = lines.pop().unwrap();
    let mut problems = Problems { problems: Vec::new() };
    for operand_str in operand_line.split_ascii_whitespace() {
        let operands = Vec::new();
        let operator = operand_str.into();
        let problem = Problem { operands, operator };
        problems.problems.push(problem);
    }
    // let problems = Problems { problems: operand_line.split_ascii_whitespace().map(|c| Problem { operands: Vec::new(), operator: c.into()}).collect()};

    for line in lines {
        let operands_row = line.split_ascii_whitespace();
        for (i, operand) in operands_row.enumerate() {
            problems.problems[i].operands.push(operand.parse().unwrap());
        }
    }

    problems
}

fn parse_input12(path: &str) -> Problems {
    let s = fs::read_to_string(path).unwrap();
    let mut lines: Vec<&str> = s.split("\n").collect();
    let operators: Vec<Operator> = lines
        .pop()
        .unwrap()
        .split_ascii_whitespace()
        .rev()
        .map(|c| c.into())
        .collect();

    let block = Block {
        block: lines.iter().map(|line| line.chars().collect()).collect(),
    };

    let operandss = block.divide_and_transpose();

    Problems {
        problems: zip(operandss, operators)
            .map(|(operands, operator)| Problem { operator, operands })
            .collect(),
    }
}

pub fn star11() {
    let problems: Problems = parse_input11("inputs/input06.txt");
    let total: i64 = problems.problems.into_iter().map(|problem| problem.evaluate()).sum();
    println!("{total}");
}

pub fn star12() {
    let problems = parse_input12("inputs/input06.txt");
    let total: i64 = problems.problems.into_iter().map(|problem| problem.evaluate()).sum();
    println!("{total}");
}
