use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
    str::FromStr,
};

use crate::solution::Solution;

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<usize> {
        let mut cols: Vec<Vec<Rock>> = vec![];
        for line in input.lines() {
            for (i, c) in line.chars().enumerate() {
                if i >= cols.len() {
                    cols.push(vec![]);
                }
                cols[i].push(c.into());
            }
        }
        let mut total_sum = 0;
        let total_rows = cols[0].len();
        for col in cols {
            let enumed = col.iter().enumerate().collect::<Vec<_>>();
            let segments = enumed.split(|r| r.1 == &Rock::Cube);
            let mut col_sum = 0;
            for segment in segments {
                if segment.is_empty() {
                    continue;
                }
                let start_row = segment[0].0;
                let start_weight = total_rows - start_row;
                for (i, _) in segment.iter().filter(|r| r.1 == &Rock::Round).enumerate() {
                    col_sum += start_weight - i;
                }
            }
            total_sum += col_sum;
        }
        Some(total_sum)
    }

    fn part2(&self, input: &str) -> Option<usize> {
        let mut dish = input.parse::<Dish>().unwrap();
        let mut cache = HashMap::new();
        for i in 0..1_000_000_000 {
            dish.cycle();
            if let Some(j) = cache.get(&dish) {
                let cycle_length = i - j;
                let remaining = 1_000_000_000 - i - 1;
                let remaining_cycles = remaining % cycle_length;
                for _ in 0..remaining_cycles {
                    dish.cycle();
                }
                break;
            }
            cache.insert(dish.clone(), i);
        }
        Some(dish.north_weight())
    }
}

enum Direction {
    North,
    West,
    South,
    East,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Dish {
    rocks: Vec<Vec<Rock>>,
}

impl Dish {
    fn north_weight(&self) -> usize {
        let mut weight = 0;
        for (i, col) in self.rocks.iter().enumerate() {
            for rock in col {
                if *rock == Rock::Round {
                    weight += self.rocks.len() - i;
                }
            }
        }
        weight
    }

    fn cycle(&mut self) {
        self.tilt(Direction::North);
        self.tilt(Direction::West);
        self.tilt(Direction::South);
        self.tilt(Direction::East);
    }

    fn tilt(&mut self, direction: Direction) {
        match direction {
            Direction::North => {
                for y in 0..self.rocks[0].len() {
                    let col = self
                        .rocks
                        .iter()
                        .map(|c| c[y])
                        .enumerate()
                        .collect::<Vec<_>>();
                    let segments = col.split(|r| r.1 == Rock::Cube).map(|s| {
                        if s.is_empty() {
                            return (0, 0, 0);
                        }
                        let start_row = s[0].0;
                        let count = s.iter().filter(|r| r.1 == Rock::Round).count();
                        (start_row, count, s.len() - count)
                    });
                    for segment in segments {
                        for i in 0..segment.1 {
                            self.rocks[segment.0 + i][y] = Rock::Round;
                        }
                        for i in 0..segment.2 {
                            self.rocks[segment.0 + segment.1 + i][y] = Rock::Empty;
                        }
                    }
                }
            }
            Direction::East => {
                for (y, row) in self.rocks.clone().iter().enumerate() {
                    let r = row.iter().enumerate().collect::<Vec<_>>();
                    let segments = r.split(|r| *r.1 == Rock::Cube).map(|s| {
                        if s.is_empty() {
                            return (0, 0, 0);
                        }
                        let start_col = s[0].0;
                        let count = s.iter().filter(|r| *r.1 == Rock::Round).count();
                        (start_col, count, s.len() - count)
                    });
                    for segment in segments {
                        for i in 0..segment.2 {
                            self.rocks[y][segment.0 + i] = Rock::Empty;
                        }
                        for i in 0..segment.1 {
                            self.rocks[y][segment.0 + segment.2 + i] = Rock::Round;
                        }
                    }
                }
            }
            Direction::South => {
                for y in 0..self.rocks[0].len() {
                    let col = self
                        .rocks
                        .iter()
                        .map(|c| c[y])
                        .enumerate()
                        .collect::<Vec<_>>();
                    let segments = col.split(|r| r.1 == Rock::Cube).map(|s| {
                        if s.is_empty() {
                            return (0, 0, 0);
                        }
                        let start_row = s[0].0;
                        let count = s.iter().filter(|r| r.1 == Rock::Round).count();
                        (start_row, count, s.len() - count)
                    });
                    for segment in segments {
                        for i in 0..segment.2 {
                            self.rocks[segment.0 + i][y] = Rock::Empty;
                        }
                        for i in 0..segment.1 {
                            self.rocks[segment.0 + segment.2 + i][y] = Rock::Round;
                        }
                    }
                }
            }
            Direction::West => {
                for (y, row) in self.rocks.clone().iter().enumerate() {
                    let r = row.iter().enumerate().collect::<Vec<_>>();
                    let segments = r.split(|r| *r.1 == Rock::Cube).map(|s| {
                        if s.is_empty() {
                            return (0, 0, 0);
                        }
                        let start_col = s[0].0;
                        let count = s.iter().filter(|r| *r.1 == Rock::Round).count();
                        (start_col, count, s.len() - count)
                    });
                    for segment in segments {
                        for i in 0..segment.1 {
                            self.rocks[y][segment.0 + i] = Rock::Round;
                        }
                        for i in 0..segment.2 {
                            self.rocks[y][segment.0 + segment.1 + i] = Rock::Empty;
                        }
                    }
                }
            }
        }
    }
}

impl FromStr for Dish {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rocks = vec![];
        for line in s.lines() {
            let mut row = vec![];
            for c in line.chars() {
                row.push(c.into());
            }
            rocks.push(row);
        }
        Ok(Self { rocks })
    }
}

impl Display for Dish {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.rocks {
            for rock in row {
                write!(f, "{}", rock)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Rock {
    Empty,
    Round,
    Cube,
}

impl From<char> for Rock {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            'O' => Self::Round,
            '#' => Self::Cube,
            _ => panic!("Unknown rock type: {}", c),
        }
    }
}

impl Display for Rock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Rock::Empty => write!(f, "."),
            Rock::Round => write!(f, "O"),
            Rock::Cube => write!(f, "#"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_input;

    #[test]
    fn test_part1_example() {
        let input = read_input(14, true, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(136))
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(14, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(108840))
    }
    #[test]
    fn test_part2_example() {
        let input = read_input(14, true, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(64))
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(14, false, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(103445))
    }
}
