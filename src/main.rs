use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(name = "advent2025")]
struct Cli {
    #[arg(value_enum)]
    star: Option<Star>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
#[allow(non_camel_case_types)]
enum Star {
    star01,
    star02,
    star03,
    star04,
    star05,
    star06,
    star07,
    star08,
    star09,
    star10,
    star11,
    star12,
    star13,
    star14,
    star15,
    star16,
    star17,
    star18,
    star19,
    star20,
    star21,
    star22,
    star23,
    star24,
}

fn main() {
    let cli = Cli::parse();
    if let Some(star) = cli.star {
        match star {
            Star::star01 => {
                advent2025::day01::star01();
            }
            Star::star02 => {
                advent2025::day01::star02();
            }
            Star::star03 => {
                advent2025::day02::star03();
            }
            Star::star04 => {
                advent2025::day02::star04();
            }
            Star::star05 => {
                advent2025::day03::star05();
            }
            Star::star06 => {
                advent2025::day03::star06();
            }
            Star::star07 => {
                advent2025::day04::star07();
            }
            Star::star08 => {
                advent2025::day04::star08();
            }
            Star::star09 => {
                advent2025::day05::star09();
            }
            Star::star10 => {
                advent2025::day05::star10();
            }
            Star::star11 => {
                advent2025::day06::star11();
            }
            Star::star12 => {
                advent2025::day06::star12();
            }
            Star::star13 => {
                advent2025::day07::star13();
            }
            Star::star14 => {
                advent2025::day07::star14();
            }
            Star::star15 => {
                advent2025::day08::star15();
            }
            Star::star16 => {
                advent2025::day08::star16();
            }
            Star::star17 => {
                advent2025::day09::star17();
            }
            Star::star18 => {
                advent2025::day09::star18();
            }
            Star::star19 => {
                advent2025::day10::star19();
            }
            Star::star20 => {
                advent2025::day10::star20();
            }
            Star::star21 => {
                advent2025::day11::star21();
            }
            Star::star22 => {
                advent2025::day11::star22();
            }
            Star::star23 => {
                advent2025::day12::star23();
            }
            Star::star24 => {
                advent2025::day12::star24();
            }
        }
    } else {
        advent2025::day01::star01();
        advent2025::day01::star02();
        advent2025::day02::star03();
        advent2025::day02::star04();
        advent2025::day03::star05();
        advent2025::day03::star06();
        advent2025::day04::star07();
        advent2025::day04::star08();
        advent2025::day05::star09();
        advent2025::day05::star10();
        advent2025::day06::star11();
        advent2025::day06::star12();
        advent2025::day07::star13();
        advent2025::day07::star14();
        advent2025::day08::star15();
        advent2025::day08::star16();
        advent2025::day09::star17();
        advent2025::day09::star18();
        advent2025::day10::star19();
        advent2025::day10::star20();
        advent2025::day11::star21();
        advent2025::day11::star22();
        advent2025::day12::star23();
        advent2025::day12::star24();
    }
}
