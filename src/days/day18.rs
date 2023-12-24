use std::str::FromStr;

use crate::solution::Solution;

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<usize> {
        Some(execute(input, Instruction::parse_part1))
    }

    fn part2(&self, input: &str) -> Option<usize> {
        Some(execute(input, Instruction::parse_part2))
    }
}

fn execute(input: &str, parser: fn(&str) -> Instruction) -> usize {
    let instructions: Vec<Instruction> = input.lines().map(parser).collect();
    // Solve using Green's theorem for polygons
    let mut x = 0;
    let mut perimeter = 0;
    let mut area = 0;
    for instruction in instructions {
        let (dy, dx) = instruction
            .direction
            .get_scaled_vector(instruction.distance);
        x += dx;
        perimeter += instruction.distance;
        area += x * dy;
    }
    (area + perimeter / 2 + 1) as usize
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    distance: isize,
}

impl Instruction {
    pub fn parse_part1(input: &str) -> Self {
        let parts = input.split_whitespace().collect::<Vec<_>>();
        let direction = parts[0].parse().unwrap();
        let distance = parts[1].parse().unwrap();
        Instruction {
            direction,
            distance,
        }
    }

    pub fn parse_part2(input: &str) -> Self {
        let color = input.split_whitespace().last().unwrap();
        let parts = color[2..color.len() - 1].to_string();
        let (distance, direction) = parts.split_at(5);
        Instruction {
            direction: direction.parse().unwrap(),
            distance: isize::from_str_radix(distance, 16).unwrap(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn get_scaled_vector(&self, scale: isize) -> (isize, isize) {
        match self {
            Direction::Down => (scale, 0),
            Direction::Up => (-scale, 0),
            Direction::Right => (0, scale),
            Direction::Left => (0, -scale),
        }
    }
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" | "3" => Ok(Direction::Up),
            "D" | "1" => Ok(Direction::Down),
            "L" | "2" => Ok(Direction::Left),
            "R" | "0" => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;
    use crate::utils::read_input;

    #[test]
    fn test_part1_example() {
        let input = read_input(18, true, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(62));
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(18, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(26857));
    }

    #[test]
    fn test_part2_example() {
        let input = read_input(18, true, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(952408144115));
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(18, false, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(129373230496292));
    }
}
