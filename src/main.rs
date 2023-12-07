mod day01;
mod day02;
mod day03;
mod solution;
use solution::Solution;

fn read_input(day: u8, example: bool) -> String {
    let filename = format!(
        "inputs/{:02}{}.txt",
        day,
        if example { "-example" } else { "" }
    );
    std::fs::read_to_string(filename).unwrap()
}

fn main() {
    let day = 3;
    let example = false;
    let input = read_input(day, example);
    let solution: Box<dyn Solution> = match day {
        1 => Box::new(day01::Day),
        2 => Box::new(day02::Day),
        3 => Box::new(day03::Day),
        _ => unreachable!("Day {} not implemented", day),
    };
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
}
