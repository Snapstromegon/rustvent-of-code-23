use std::str::FromStr;

use crate::solution::Solution;

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<usize> {
        let trees: Vec<PredictionTree> = input.lines().map(|l| l.parse().unwrap()).collect();
        let predictions = trees.iter().map(|t| t.get_prediction()).collect::<Vec<_>>();
        Some(predictions.iter().sum::<i64>() as usize)
    }

    fn part2(&self, input: &str) -> Option<usize> {
        let trees: Vec<PredictionTree> = input.lines().map(|l| l.parse().unwrap()).collect();
        let predictions = trees
            .iter()
            .map(|t| t.get_postdiction())
            .collect::<Vec<_>>();
        Some(predictions.iter().sum::<i64>() as usize)
    }
}

#[derive(Debug)]
struct PredictionTree {
    nodes: Vec<i64>,
    child: Option<Box<PredictionTree>>,
}

impl PredictionTree {
    fn new(nodes: Vec<i64>) -> Self {
        if nodes.len() < 2 {
            return Self { nodes, child: None };
        }
        let mut child_nodes = Vec::new();
        let mut node_iter = nodes.iter();
        let mut last = node_iter.next().unwrap();
        for node in node_iter {
            child_nodes.push(*node - *last);
            last = node;
        }
        Self {
            nodes,
            child: if child_nodes.iter().all(|x| *x == 0) {
                None
            } else {
                Some(Box::new(PredictionTree::new(child_nodes)))
            },
        }
    }

    pub fn get_prediction(&self) -> i64 {
        if let Some(child) = &self.child {
            child.get_prediction() + self.nodes.iter().last().unwrap()
        } else {
            self.nodes[0]
        }
    }

    pub fn get_postdiction(&self) -> i64 {
        if let Some(child) = &self.child {
            self.nodes.first().unwrap() - child.get_postdiction()
        } else {
            self.nodes[0]
        }
    }
}

impl FromStr for PredictionTree {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(
            s.split_whitespace().map(|s| s.parse().unwrap()).collect(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_input;

    #[test]
    fn test_part1_example() {
        let input = read_input(9, true, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(114));
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(9, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(1681758908));
    }

    #[test]
    fn test_part2_example() {
        let input = read_input(9, true, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(2));
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(9, false, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(803));
    }
}
