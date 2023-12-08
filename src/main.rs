use clap::Parser;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day08;
mod solution;
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

fn read_input(day: usize, example: bool) -> String {
    let filename = format!(
        "inputs/{:02}{}.txt",
        day,
        if example { "-example" } else { "" }
    );
    std::fs::read_to_string(filename).unwrap()
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
        8 => Some(Box::new(day08::Day)),
        _ => None,
    };
    if let Some(solution) = solution {
        println!("Day {}:", day);
        let input = read_input(day, example);
        let start = std::time::Instant::now();
        let result = solution.part1(&input);
        let duration = start.elapsed();
        if let Some(result) = result {
            println!("Part 1: {} (took {:?})", result, duration);
        } else {
            println!("Part 1: not implemented for day {day}");
        }
        let start = std::time::Instant::now();
        let result = solution.part2(&input);
        let duration = start.elapsed();
        if let Some(result) = result {
            println!("Part 2: {} (took {:?})", result, duration);
        } else {
            println!("Part 2: not implemented for day {day}");
        }
    } else {
        println!("Day {} not implemented", day);
    }
}
