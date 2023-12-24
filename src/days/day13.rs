use std::{fmt::Display, str::FromStr};

use crate::solution::Solution;

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<usize> {
        let maps: Vec<Map> = input
            .split("\n\n")
            .map(|block| block.parse().unwrap())
            .collect();
        Some(maps.iter().map(|m| m.get_summary()).sum())
    }

    fn part2(&self, input: &str) -> Option<usize> {
        let maps: Vec<Map> = input
            .split("\n\n")
            .map(|block| block.parse().unwrap())
            .collect();
        Some(
            maps.iter()
                .map(|m| {
                    let unsmudged = m.unsmudge();
                    unsmudged.get_summary()
                })
                .sum(),
        )
    }
}

#[derive(Debug)]
struct Map {
    cells: Vec<Vec<Cell>>,
    ignore_row_mirror: Option<usize>,
    ignore_col_mirror: Option<usize>,
}

impl Map {
    fn test_row_mirror(&self, start: usize) -> bool {
        let max_range = start.min(self.cells.len() - 1 - 1 - start);
        for i in 1..=max_range {
            if self.cells[start - i] != self.cells[start + 1 + i] {
                return false;
            }
        }
        true
    }

    pub fn rows_above_mirror(&self) -> usize {
        for (index, row) in self.cells.iter().enumerate() {
            if Some(index + 1) != self.ignore_row_mirror
                && self.cells.len() > index + 1
                && *row == self.cells[index + 1]
                && self.test_row_mirror(index)
            {
                return index + 1;
            }
        }

        0
    }

    fn get_col(&self, i: usize) -> Option<Vec<Cell>> {
        if self.cells.is_empty() || self.cells[0].len() <= i {
            return None;
        }
        Some(self.cells.iter().map(|row| row[i]).collect())
    }

    fn test_col_mirror(&self, start: usize) -> bool {
        if self.cells.is_empty() {
            return true;
        }
        let max_range = start.min(self.cells[0].len() - 1 - 1 - start);
        for i in 1..=max_range {
            if self.get_col(start - i) != self.get_col(start + 1 + i) {
                return false;
            }
        }
        true
    }

    pub fn cols_left_of_mirror(&self) -> usize {
        if self.cells.is_empty() {
            return 0;
        }
        for index in 0..self.cells[0].len() {
            if Some(index + 1) != self.ignore_col_mirror
                && self.get_col(index) == self.get_col(index + 1)
                && self.test_col_mirror(index)
            {
                return index + 1;
            }
        }

        0
    }

    pub fn get_summary(&self) -> usize {
        self.rows_above_mirror() * 100 + self.cols_left_of_mirror()
    }

    pub fn unsmudge(&self) -> Self {
        let old_row_mirror = self.rows_above_mirror();
        let old_col_mirror = self.cols_left_of_mirror();
        for (row, line) in self.cells.iter().enumerate() {
            for col in 0..line.len() {
                let mut candidate = Self {
                    cells: self.cells.clone(),
                    ignore_row_mirror: Some(old_row_mirror),
                    ignore_col_mirror: Some(old_col_mirror),
                };
                candidate.flip_cell(row, col);
                if candidate.get_summary() != 0 {
                    return candidate;
                }
            }
        }
        unreachable!("No smudge found!")
    }

    fn flip_cell(&mut self, row: usize, col: usize) {
        self.cells[row][col].flip()
    }
}

impl FromStr for Map {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            cells: s
                .lines()
                .map(|line| line.chars().map(|c| c.into()).collect())
                .collect(),
            ignore_row_mirror: None,
            ignore_col_mirror: None,
        })
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.cells {
            for cell in row {
                write!(f, "{cell}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Cell {
    Ash,
    Rock,
}

impl Cell {
    pub fn flip(&mut self) {
        *self = match self {
            Self::Ash => Self::Rock,
            Self::Rock => Self::Ash,
        }
    }
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Ash,
            '#' => Self::Rock,
            _ => unreachable!("Invalid cell type"),
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Ash => '.',
                Self::Rock => '#',
            }
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::utils::read_input;

    #[test]
    fn test_part1_example() {
        let input = read_input(13, true, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(405))
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(13, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(34100))
    }

    #[test]
    fn test_part2_example() {
        let input = read_input(13, true, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(400))
    }

    #[test]
    fn test_part2_challenge() {
        let input = read_input(13, false, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(33106))
    }
}
