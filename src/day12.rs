use std::collections::HashMap;
use std::fmt;
use std::{fmt::Display, str::FromStr};

use crate::solution::Solution;

pub struct Day;

impl Solution for Day {
  /// ```
  /// # use rustvent::utils::read_input;
  /// # use rustvent::solution::Solution;
  /// # use rustvent::day12::Day;
  /// let input = read_input(12, true, 1).unwrap();
  /// assert_eq!(Day.part1(&input), Some(21))
  /// ```
  ///
  /// ```
  /// # use rustvent::utils::read_input;
  /// # use rustvent::solution::Solution;
  /// # use rustvent::day12::Day;
  /// let input = read_input(12, false, 1).unwrap();
  /// assert_eq!(Day.part1(&input), Some(7541))
  /// ```
    fn part1(&self, input: &str) -> Option<usize> {
        let spring_sets: Vec<SpringSet> = input.lines().map(|line| line.parse().unwrap()).collect();
        Some(spring_sets.iter().map(|s| s.count_possibles()).sum())
    }

  /// ```
  /// # use rustvent::utils::read_input;
  /// # use rustvent::solution::Solution;
  /// # use rustvent::day12::Day;
  /// let input = read_input(12, true, 2).unwrap();
  /// assert_eq!(Day.part2(&input), Some(525152))
  /// ```
  ///
  /// ```
  /// # use rustvent::utils::read_input;
  /// # use rustvent::solution::Solution;
  /// # use rustvent::day12::Day;
  /// let input = read_input(12, false, 2).unwrap();
  /// assert_eq!(Day.part2(&input), Some(17485169859432))
  /// ```
    fn part2(&self, input: &str) -> Option<usize> {
        let spring_sets: Vec<SpringSet> = input.lines().map(|line| line.parse().unwrap()).collect();
        let unfolded: Vec<SpringSet> = spring_sets.iter().map(|s| s.unfold()).collect();
        Some(unfolded.iter().map(|s| s.count_possibles()).sum())
    }
}

fn memo_count(
    statuses: Vec<SpringStatus>,
    broken_chains: Vec<usize>,
    cache: &mut HashMap<(Vec<SpringStatus>, Vec<usize>), usize>,
) -> usize {
    let clone = (statuses.clone(), broken_chains.clone());
    if let Some(count) = cache.get(&clone) {
        return *count;
    }
    let res = count_ways(statuses, broken_chains, cache);
    cache.insert(clone.clone(), res);
    res
}

fn count_ways(
    statuses: Vec<SpringStatus>,
    broken_chains: Vec<usize>,
    cache: &mut HashMap<(Vec<SpringStatus>, Vec<usize>), usize>,
) -> usize {
    let res = if statuses.is_empty() {
        if broken_chains.is_empty() {
            1
        } else {
            0
        }
    } else if broken_chains.is_empty() {
        if statuses.iter().any(|s| *s == SpringStatus::Broken) {
            0
        } else {
            1
        }
    } else if statuses.len() < broken_chains.len() + broken_chains.iter().sum::<usize>() - 1 {
        0
    } else {
        match statuses[0] {
            SpringStatus::Ok => memo_count(statuses[1..].to_vec(), broken_chains, cache),
            SpringStatus::Broken => {
                let (run, remaining) = broken_chains.split_first().unwrap();
                let elems: Vec<_> = statuses.iter().take(*run).collect();
                if elems.len() < *run
                    || (*run < statuses.len() && statuses[*run] == SpringStatus::Broken)
                    || elems.iter().any(|s| **s == SpringStatus::Ok)
                {
                    0
                } else if run < &statuses.len() {
                    let next_statuses = &statuses[run + 1..];
                    memo_count(next_statuses.to_vec(), remaining.to_vec(), cache)
                } else {
                    1
                }
            }
            SpringStatus::Unknown => {
                let statuses_ok: Vec<SpringStatus> = [SpringStatus::Ok]
                    .into_iter()
                    .chain(statuses.clone().iter().skip(1).cloned())
                    .collect();
                let statuses_broken: Vec<SpringStatus> = [SpringStatus::Broken]
                    .into_iter()
                    .chain(statuses.clone().iter().skip(1).cloned())
                    .collect();
                memo_count(statuses_ok.to_vec(), broken_chains.clone(), cache)
                    + memo_count(statuses_broken.to_vec(), broken_chains.clone(), cache)
            }
        }
    };
    res
}

#[derive(Debug, Clone)]
struct SpringSet {
    statuses: Vec<SpringStatus>,
    broken_chains: Vec<usize>,
}

impl SpringSet {
    pub fn unfold(&self) -> Self {
        Self {
            statuses: (0..5)
                .map(|_| self.statuses.clone())
                .collect::<Vec<_>>()
                .join(&SpringStatus::Unknown),
            broken_chains: (0..5)
                .map(|_| self.broken_chains.clone())
                .flatten()
                .collect::<Vec<_>>(),
        }
    }

    pub fn count_possibles(&self) -> usize {
        memo_count(
            self.statuses.clone(),
            self.broken_chains.clone(),
            &mut HashMap::new(),
        )
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

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
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
                Self::Ok => ".",
                Self::Broken => "#",
                Self::Unknown => "?",
            }
        )
    }
}

impl Ord for SpringStatus {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Ok, Self::Ok) => std::cmp::Ordering::Equal,
            (Self::Ok, Self::Broken) => std::cmp::Ordering::Less,
            (Self::Ok, Self::Unknown) => std::cmp::Ordering::Less,
            (Self::Broken, Self::Ok) => std::cmp::Ordering::Greater,
            (Self::Broken, Self::Broken) => std::cmp::Ordering::Equal,
            (Self::Broken, Self::Unknown) => std::cmp::Ordering::Less,
            (Self::Unknown, Self::Ok) => std::cmp::Ordering::Greater,
            (Self::Unknown, Self::Broken) => std::cmp::Ordering::Greater,
            (Self::Unknown, Self::Unknown) => std::cmp::Ordering::Equal,
        }
    }
}

impl PartialOrd for SpringStatus {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
