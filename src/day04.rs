use std::{collections::HashMap, fs};

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct Position {
    row: isize,
    col: isize,
}

impl Position {
    fn neighbors(&self) -> Vec<Position> {
        let Position {
            row: old_row,
            col: old_col,
        } = self;
        let mut neighbors = Vec::new();
        for drow in -1..2 {
            let row = old_row + drow;
            for dcol in -1..2 {
                let col = old_col + dcol;
                if row != *old_row || col != *old_col {
                    neighbors.push(Position { row, col })
                }
            }
        }
        neighbors
    }
}

#[derive(Debug)]
enum Contents {
    Empty,
    Roll,
}

impl From<char> for Contents {
    fn from(item: char) -> Self {
        match item {
            '@' => Contents::Roll,
            '.' => Contents::Empty,
            _ => panic!("invalid character!"),
        }
    }
}

#[derive(Debug)]
struct Map {
    map: HashMap<Position, Contents>,
}

impl Map {
    fn neighboring_papers(&self, pos: &Position) -> u8 {
        let neighbors = pos.neighbors();
        let mut n_paper = 0;
        for neighbor in neighbors {
            if let Some(contents) = self.map.get(&neighbor) {
                match contents {
                    Contents::Roll => n_paper += 1,
                    Contents::Empty => {}
                }
            }
        }
        n_paper
    }

    fn accessible(&self, pos: &Position, threshold: u8) -> bool {
        if let Some(contents) = self.map.get(pos) {
            match contents {
                Contents::Empty => false,
                Contents::Roll => self.neighboring_papers(pos) < threshold,
            }
        } else {
            false
        }
    }

    fn all_accessible(&self, threshold: u8) -> Vec<Position> {
        self.map
            .keys()
            .filter(|pos| self.accessible(pos, threshold))
            .cloned()
            .collect()
    }
}

fn parse_input(path: &str) -> Map {
    let mut map: HashMap<Position, Contents> = HashMap::new();
    let s = fs::read_to_string(path).unwrap();
    for (row, line) in s.split("\n").enumerate() {
        for (col, c) in line.char_indices() {
            let pos = Position {
                row: row as isize,
                col: col as isize,
            };
            map.insert(pos, c.into());
        }
    }
    Map { map }
}

pub fn star07() {
    let map = parse_input("inputs/input04.txt");
    let n_accessible: u32 = map
        .map
        .keys()
        .map(|pos| map.accessible(pos, 4) as u32)
        .sum();
    println!("{n_accessible}");
}

pub fn star08() {
    let mut map = parse_input("inputs/input04.txt");
    let mut n_removed = 0;
    let mut positions = map.all_accessible(4);
    while positions.len() > 0 {
        for pos in positions.into_iter() {
            map.map.remove(&pos);
            n_removed += 1;
        }
        positions = map.all_accessible(4);
    }
    println!("{n_removed}");
}
