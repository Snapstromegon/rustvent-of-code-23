use std::{collections::HashMap, str::FromStr};

use crate::solution::Solution;

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<usize> {
        let plan: Plan = input.parse().unwrap();
        plan.parts.iter().map(|part| part.id).sum::<usize>().into()
    }

    fn part2(&self, input: &str) -> Option<usize> {
        let plan: Plan = input.parse().unwrap();
        plan.get_gears()
            .iter()
            .map(|(a, b)| a.id * b.id)
            .sum::<usize>()
            .into()
    }
}

#[derive(Debug)]
struct Plan {
    parts: Vec<Part>,
    markers: Vec<(char, usize, usize)>,
}

impl Plan {
    fn related_markers(&self, part: &Part) -> Vec<(char, usize, usize)> {
        self.markers
            .iter()
            .filter(|marker| {
                let min_col = if part.col == 0 { 0 } else { part.col - 1 };
                let max_col = part.col + part.length;
                let min_row = if part.row == 0 { 0 } else { part.row - 1 };
                let max_row = part.row + 1;
                (min_col..=max_col).contains(&marker.2) && (min_row..=max_row).contains(&marker.1)
            })
            .copied()
            .collect()
    }

    fn get_gears(&self) -> Vec<(Part, Part)> {
        let mut connections: HashMap<(char, usize, usize), Vec<Part>> = HashMap::new();
        for part in &self.parts {
            let touched_gears: Vec<(char, usize, usize)> = self
                .related_markers(part)
                .iter()
                .filter(|(c, _, _)| *c == '*')
                .cloned()
                .collect();
            for gear in touched_gears {
                if let std::collections::hash_map::Entry::Vacant(e) = connections.entry(gear) {
                    e.insert(vec![part.clone()]);
                } else {
                    connections.get_mut(&gear).unwrap().push(part.clone());
                }
            }
        }
        connections
            .iter()
            .filter(|(_, v)| v.len() == 2)
            .map(|(_, v)| (v[0].clone(), v[1].clone()))
            .collect()
    }
}

impl FromStr for Plan {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let markers = s
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars().enumerate().filter_map(move |(col, c)| {
                    if !c.is_numeric() && c != '.' {
                        Some((c, row, col))
                    } else {
                        None
                    }
                })
            })
            .collect::<Vec<_>>();
        let mut plan = Plan {
            parts: Vec::new(),
            markers,
        };

        // parse each line like "..452....45677..." into a list of numbers like [452, 45677]
        let parts: Vec<Part> = s
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                let mut chars = line.chars().enumerate().peekable();
                let mut numbers = Vec::new();
                let mut number = 0;
                let mut start = usize::MAX;
                while chars.peek().is_some() {
                    let (col, c) = chars.next().unwrap();
                    if c.is_numeric() {
                        number = number * 10 + c.to_digit(10).unwrap() as usize;
                        if start == usize::MAX {
                            start = col;
                        }
                    } else if start != usize::MAX {
                        numbers.push(Part {
                            id: number,
                            row,
                            col: start,
                            length: col - start,
                        });
                        number = 0;
                        start = usize::MAX;
                    }
                }
                if start != usize::MAX {
                    numbers.push(Part {
                        id: number,
                        row,
                        col: start,
                        length: line.len() - start,
                    });
                }
                numbers
            })
            .filter(|part| !plan.related_markers(part).is_empty())
            .collect();

        plan.parts.extend(parts);
        Ok(plan)
    }
}

#[derive(Debug, Clone)]
struct Part {
    id: usize,
    row: usize,
    col: usize,
    length: usize,
}
