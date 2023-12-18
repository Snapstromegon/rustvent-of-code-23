use std::str::FromStr;

use crate::solution::Solution;

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<usize> {
        Some(execute(input, 1))
    }

    fn part2(&self, input: &str) -> Option<usize> {
        None
    }
}

fn execute(input: &str, part: usize) -> usize {
    let instructions: Vec<Instruction> = input
        .lines()
        .map(|line| match part {
            1 => Instruction::parse_part1(line),
            2 => Instruction::parse_part2(line),
            _ => unreachable!(),
        })
        .collect();
    for instruction in &instructions {
        println!("{:?}", instruction);
    }
    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut curr_x = 0;
    let mut curr_y = 0;
    for instruction in &instructions {
        match instruction.direction {
            Direction::Up => curr_y -= instruction.distance,
            Direction::Down => curr_y += instruction.distance,
            Direction::Left => curr_x -= instruction.distance,
            Direction::Right => curr_x += instruction.distance,
        }
        if curr_x < min_x {
            min_x = curr_x;
        }
        if curr_y < min_y {
            min_y = curr_y;
        }
        if curr_x > max_x {
            max_x = curr_x;
        }
        if curr_y > max_y {
            max_y = curr_y;
        }
    }
    let mut field =
        vec![vec![FieldState::Empty; (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize];
    let mut curr_x = -min_x;
    let mut curr_y = -min_y;
    field[curr_y as usize][curr_x as usize] = FieldState::Edge;
    for instruction in instructions {
        let dir = match instruction.direction {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        };
        for _ in 0..instruction.distance {
            curr_y += dir.0;
            curr_x += dir.1;
            field[curr_y as usize][curr_x as usize] = FieldState::Edge;
        }
    }

    let mut flood_from = vec![];
    for row in 0..field.len() {
        if field[row][0] == FieldState::Empty {
            flood_from.push((row, 0));
        }
        if field[row][field[0].len() - 1] == FieldState::Empty {
            flood_from.push((row, field[0].len() - 1));
        }
    }
    for col in 0..field[0].len() {
        if field[0][col] == FieldState::Empty {
            flood_from.push((0, col));
        }
        if field[field.len() - 1][col] == FieldState::Empty {
            flood_from.push((field.len() - 1, col));
        }
    }
    while !flood_from.is_empty() {
        let mut new_flood_from = vec![];
        for (y, x) in flood_from {
            if field[y][x] == FieldState::Empty {
                field[y][x] = FieldState::Ground;
                if y > 0 && field[y - 1][x] == FieldState::Empty {
                    new_flood_from.push((y - 1, x));
                }
                if y < field.len() - 1 && field[y + 1][x] == FieldState::Empty {
                    new_flood_from.push((y + 1, x));
                }
                if x > 0 && field[y][x - 1] == FieldState::Empty {
                    new_flood_from.push((y, x - 1));
                }
                if x < field[y].len() - 1 && field[y][x + 1] == FieldState::Empty {
                    new_flood_from.push((y, x + 1));
                }
            }
        }
        flood_from = new_flood_from;
    }

    for row in &mut field {
        for col in &mut row.iter_mut() {
            if *col == FieldState::Empty {
                *col = FieldState::Dug;
            }
        }
    }

    field
        .iter()
        .flat_map(|row| {
            row.iter()
                .filter(|&&col| col == FieldState::Dug || col == FieldState::Edge)
        })
        .count()
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum FieldState {
    Edge,
    Dug,
    Ground,
    Empty,
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
        assert_eq!(Day.part2(&input), None);
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(18, false, 2).unwrap();
        assert_eq!(Day.part2(&input), None);
    }
}
