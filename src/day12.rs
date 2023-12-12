use std::fmt;
use std::{fmt::Display, str::FromStr};

use crate::solution::Solution;

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<usize> {
        let spring_sets: Vec<SpringSet> = input.lines().map(|line| line.parse().unwrap()).collect();
        Some(spring_sets.iter().map(|s| s.count_possibles()).sum())
    }

    fn part2(&self, input: &str) -> Option<usize> {
        let spring_sets: Vec<SpringSet> = input.lines().map(|line| line.parse().unwrap()).collect();
        let unfolded: Vec<SpringSet> = spring_sets.iter().map(|s| s.unfold()).collect();
        Some(
            unfolded
                .iter()
                .map(|s| {
                    println!("Processing {s}");
                    s.count_possibles()
                })
                .sum(),
        )
    }
}

#[derive(Debug, Clone)]
struct SpringSet {
    statuses: Vec<SpringStatus>,
    broken_chains: Vec<usize>,
}

impl SpringSet {
    pub fn unfold(&self) -> Self {
        let mut statuses = vec![];
        let mut broken_chains = vec![];
        for _i in 0..5 {
            statuses.extend(self.statuses.iter().cloned());
            broken_chains.extend(self.broken_chains.iter().cloned());
        }
        Self {
            statuses,
            broken_chains,
        }
    }

    pub fn number_of_unknowns(&self) -> usize {
        self.statuses
            .iter()
            .filter(|status| **status == SpringStatus::Unknown)
            .count()
    }

    pub fn count_possibles(&self) -> usize {
        if self.number_of_unknowns() == 0 {
            if self.is_valid() {
                // println!("Counting for OK {:?}", self);
                1
            } else {
                // println!("Counting for BROKEN {:?}", self);
                0
            }
        } else {
            let first_unknown_index = self
                .statuses
                .iter()
                .enumerate()
                .filter(|(_, s)| **s == SpringStatus::Unknown)
                .map(|(i, _)| i)
                .next()
                .unwrap();
            let mut clone_ok = self.clone();
            clone_ok.statuses[first_unknown_index] = SpringStatus::Ok;
            let mut clone_broken = self.clone();
            clone_broken.statuses[first_unknown_index] = SpringStatus::Broken;
            clone_ok.count_possibles() + clone_broken.count_possibles()
        }
    }

    pub fn is_valid(&self) -> bool {
        if self.number_of_unknowns() > 0 {
            return false;
        }
        let mut stat = self.statuses.iter();
        let mut chains = self.broken_chains.iter();
        while let Some(broken_chain) = chains.next() {
            // Fast forward to next broken
            let mut next = stat.next();
            while let Some(SpringStatus::Ok) = next {
                next = stat.next()
            }
            if next == None {
                return false;
            }
            // check that at least the right number of brokens exists
            for _ in 1..*broken_chain {
                if let Some(SpringStatus::Broken) = stat.next() {
                } else {
                    return false;
                }
            }
            // check that the next one is not broken
            if matches!(stat.next(), Some(&SpringStatus::Broken)) {
                return false;
            }
        }
        stat.all(|s| *s == SpringStatus::Ok)
    }
}

impl FromStr for SpringSet {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (stats, chains) = s.split_once(' ').unwrap();
        Ok(Self {
            statuses: stats.chars().map(|c| c.into()).collect(),
            broken_chains: chains.split(',').map(|s| s.parse().unwrap()).collect(),
        })
    }
}

impl Display for SpringSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for status in &self.statuses {
            write!(f, "{}", status)?;
        }
        write!(f, " - ")?;
        for broken in &self.broken_chains {
            write!(f, "{}", broken)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum SpringStatus {
    Ok,
    Broken,
    Unknown,
}

impl From<char> for SpringStatus {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Ok,
            '#' => Self::Broken,
            '?' => Self::Unknown,
            _ => unreachable!("Invalid spring status"),
        }
    }
}

impl Display for SpringStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Ok => "✔",
                Self::Broken => "X",
                Self::Unknown => "?",
            }
        )
    }
}
