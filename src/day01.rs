use std::{fs, i32};

#[derive(Debug)]
enum Command {
    Left(i32),
    Right(i32),
}

struct Position {
    position: i32,
}

impl Position {
    fn rotate(&mut self, command: Command) {
        match command {
            Command::Left(distance) => self.position = (self.position + distance).rem_euclid(100),
            Command::Right(distance) => self.position = (self.position - distance).rem_euclid(100),
        }
    }

    fn rotate_and_count(&mut self, command: Command) -> i32 {
        let mut count = 0;
        match command {
            Command::Left(distance) => {
                if self.position == 0 {
                    count -= 1;
                }
                self.position = self.position - distance;
                while self.position < 0 {
                    self.position += 100;
                    count += 1;
                }
                if self.position == 0 {
                    count += 1;
                }
            }
            Command::Right(distance) => {
                self.position = self.position + distance;
                while self.position > 99 {
                    self.position -= 100;
                    count += 1;
                }
            }
        }
        return count;
    }
}

impl From<&str> for Command {
    fn from(item: &str) -> Self {
        let split = item.split_at(1);
        let direction = split.0;
        let distance = i32::from_str_radix(split.1, 10).unwrap();
        match direction {
            "L" => Command::Left(distance),
            "R" => Command::Right(distance),
            _ => panic!("direction must be 'L' or 'R'!"),
        }
    }
}

fn parse_input(path: &str) -> Vec<Command> {
    fs::read_to_string(path)
        .unwrap()
        .split("\n")
        .map(|x| x.into())
        .collect()
}

pub fn star01() {
    let mut n_zero = 0;
    let mut p = Position { position: 50 };
    let commands = parse_input("inputs/input01.txt");
    for command in commands {
        p.rotate(command);
        if p.position == 0 {
            n_zero += 1;
        }
    }
    println!("{n_zero}");
}

pub fn star02() {
    let mut n_zero = 0;
    let mut p = Position { position: 50 };
    let commands = parse_input("inputs/input01.txt");
    for command in commands {
        n_zero += p.rotate_and_count(command);
    }
    println!("{n_zero}");
}
