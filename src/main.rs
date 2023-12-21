use clap::Parser;

use rustvent::*;
pub mod utils;
use solution::Solution;

#[derive(Parser, Debug)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    day: usize,

    /// Number of times to greet
    #[arg(short, long, default_value_t = false)]
    example: bool,
}

fn main() {
    let args = Args::parse();
    let day = args.day;
    let example = args.example;
    let solution: Option<Box<dyn Solution>> = match day {
        1 => Some(Box::new(day01::Day)),
        2 => Some(Box::new(day02::Day)),
        3 => Some(Box::new(day03::Day)),
        4 => Some(Box::new(day04::Day)),
        5 => Some(Box::new(day05::Day)),
        6 => Some(Box::new(day06::Day)),
        7 => Some(Box::new(day07::Day)),
        8 => Some(Box::new(day08::Day)),
        9 => Some(Box::new(day09::Day)),
        10 => Some(Box::new(day10::Day)),
        11 => Some(Box::new(day11::Day)),
        12 => Some(Box::new(day12::Day)),
        13 => Some(Box::new(day13::Day)),
        14 => Some(Box::new(day14::Day)),
        15 => Some(Box::new(day15::Day)),
        16 => Some(Box::new(day16::Day)),
        17 => Some(Box::new(day17::Day)),
        18 => Some(Box::new(day18::Day)),
        19 => Some(Box::new(day19::Day)),
        20 => Some(Box::new(day20::Day)),
        _ => None,
    };
    if let Some(solution) = solution {
        println!("Day {}:", day);
        if let Some(input) = utils::read_input(day, example, 1) {
            let start = std::time::Instant::now();
            let result = solution.part1(&input);
            let duration = start.elapsed();
            if let Some(result) = result {
                println!("Part 1: {} (took {:?})", result, duration);
            } else {
                println!("Part 1: not implemented for day {day}");
            }
        } else {
            println!("Part 1: no input found for day {day} (example: {example})");
        }
        if let Some(input) = utils::read_input(day, example, 2) {
            let start = std::time::Instant::now();
            let result = solution.part2(&input);
            let duration = start.elapsed();
            if let Some(result) = result {
                println!("Part 2: {} (took {:?})", result, duration);
            } else {
                println!("Part 2: not implemented for day {day}");
            }
        } else {
            println!("Part 2: no input found for day {day} (example: {example})");
        }
    } else {
        println!("Day {} not implemented", day);
    }
}
