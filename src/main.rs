use clap::Parser;

use crate::cli::Cli;
use crate::parse::parse_input;
use crate::solutions::*;

mod cli;
mod grid_utils;
mod parse;
mod solutions;

fn main() {
    let args = Cli::parse();

    // default to last day
    let day = args.day.unwrap_or(14);

    match day {
        1 => {
            day1::solve(&parse_input("inputs/input_day1.txt"));
            day1b::solve(&parse_input("inputs/input_day1.txt"));
        }
        2 => {
            day2::solve(&parse_input("inputs/input_day2.txt"));
        }
        3 => {
            day3::solve(&parse_input("inputs/input_day3.txt"));
        }
        4 => {
            day4::solve(&parse_input("inputs/input_day4.txt"));
        }
        5 => {
            day5::solve(&parse_input("inputs/input_day5.txt"));
        }
        6 => {
            day6::solve(&parse_input("inputs/input_day6.txt"));
        }
        7 => {
            day7::solve(&parse_input("inputs/input_day7.txt"));
        }
        8 => {
            day8::solve(&parse_input("inputs/input_day8.txt"));
        }
        9 => {
            day9::solve(&parse_input("inputs/input_day9.txt"));
        }
        10 => {
            day10::solve(&parse_input("inputs/input_day10.txt"));
        }
        11 => {
            day11::solve(&parse_input("inputs/input_day11.txt"));
        }
        12 => {
            day12::solve(&parse_input("inputs/input_day12.txt"));
        }
        13 => {
            day13::solve(&parse_input("inputs/input_day13.txt"));
        }
        14 => {
            day14::solve(&parse_input("inputs/input_day14.txt"));
        }
        _ => println!("Specified day not yet implemented"),
    }
}
