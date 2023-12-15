use std::str::FromStr;

use crate::solution::Solution;

pub struct Day;

impl Solution for Day {
    /// ```
    /// # use rustvent::utils::read_input;
    /// # use rustvent::solution::Solution;
    /// # use rustvent::day15::Day;
    /// let input = read_input(15, true, 1).unwrap();
    /// assert_eq!(Day.part1(&input), Some(1320))
    /// ```
    ///
    /// ```
    /// # use rustvent::utils::read_input;
    /// # use rustvent::solution::Solution;
    /// # use rustvent::day15::Day;
    /// let input = read_input(15, false, 1).unwrap();
    /// assert_eq!(Day.part1(&input), Some(495972))
    /// ```
    fn part1(&self, input: &str) -> Option<usize> {
        Some(input.split(',').map(xmas_hash).sum())
    }

    /// ```
    /// # use rustvent::utils::read_input;
    /// # use rustvent::solution::Solution;
    /// # use rustvent::day15::Day;
    /// let input = read_input(15, true, 2).unwrap();
    /// assert_eq!(Day.part2(&input), Some(145))
    /// ```
    ///
    /// ```
    /// # use rustvent::utils::read_input;
    /// # use rustvent::solution::Solution;
    /// # use rustvent::day15::Day;
    /// let input = read_input(15, false, 2).unwrap();
    /// assert_eq!(Day.part2(&input), Some(245223))
    /// ```
    fn part2(&self, input: &str) -> Option<usize> {
        let operations: Vec<Operation> = input.split(',').map(|s| s.parse().unwrap()).collect();
        let mut boxes: Vec<LensBox> = (0..256).map(|_| LensBox { lenses: vec![] }).collect();
        for op in operations {
            let box_number = op.box_number();
            boxes[box_number].apply(op);
        }
        Some(
            boxes
                .iter()
                .enumerate()
                .map(|(i, lens_box)| (i + 1) * lens_box.focusing_power())
                .sum(),
        )
    }
}

fn xmas_hash(input: &str) -> usize {
    input
        .chars()
        .map(u32::from)
        .fold(0, |acc, x| ((acc + x) * 17) % 256) as usize
}

#[derive(Debug)]
struct LensBox {
    lenses: Vec<Lens>,
}

impl LensBox {
    fn pos_lens_with_label(&self, label: &str) -> Option<usize> {
        self.lenses.iter().position(|lens| lens.label == label)
    }

    pub fn apply(&mut self, op: Operation) {
        match (op.clone(), self.pos_lens_with_label(op.label())) {
            (Operation::Remove(_), None) => {}
            (Operation::Remove(_), Some(pos)) => {
                self.lenses.remove(pos);
            }
            (Operation::Set(_, lens), None) => self.lenses.push(lens),
            (Operation::Set(_, lens), Some(pos)) => {
                self.lenses.push(lens);
                self.lenses.swap_remove(pos);
            }
        }
    }

    pub fn focusing_power(&self) -> usize {
        self.lenses
            .iter()
            .enumerate()
            .map(|(i, lens)| (i + 1) * lens.focal_length)
            .sum()
    }
}

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    focal_length: usize,
}

impl FromStr for Lens {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (label, f_length) = s.split_once('=').unwrap();
        Ok(Self {
            label: label.to_string(),
            focal_length: f_length.parse().unwrap(),
        })
    }
}

#[derive(Debug, Clone)]
enum Operation {
    Remove(String),
    Set(String, Lens),
}

impl Operation {
    pub fn label(&self) -> &str {
        match self {
            Self::Remove(x) => x,
            Self::Set(x, _) => x,
        }
    }

    pub fn box_number(&self) -> usize {
        xmas_hash(self.label())
    }
}

impl FromStr for Operation {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.ends_with('-') {
            Ok(Self::Remove(s[0..s.len() - 1].to_string()))
        } else {
            let label = s.split_once('=').unwrap().0;
            Ok(Self::Set(label.to_string(), s.parse().unwrap()))
        }
    }
}
