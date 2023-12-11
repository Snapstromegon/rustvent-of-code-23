use std::collections::HashSet;

use crate::solution::Solution;

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<usize> {
        let sky: Sky = Sky::from_str(input, 2);
        Some(sky.get_distances())
    }

    fn part2(&self, input: &str) -> Option<usize> {
        let sky: Sky = Sky::from_str(input, 1_000_000);
        Some(sky.get_distances())
    }
}

#[derive(Debug)]
struct Sky {
    galaxies: Vec<(usize, usize)>,
}

impl Sky {
    pub fn get_distances(&self) -> usize {
        self.galaxies
            .iter()
            .enumerate()
            .map(|(i, galaxy)| {
                let mut res = 0;
                for other in self.galaxies.iter().skip(i) {
                    res += galaxy.0.abs_diff(other.0) + galaxy.1.abs_diff(other.1);
                }
                res
            })
            .sum()
    }

    fn from_str(s: &str, expansion: usize) -> Self {
        let sky: Vec<Vec<char>> = s.lines().map(|s| s.chars().collect()).collect();

        let empty_rows: HashSet<usize> = sky
            .iter()
            .enumerate()
            .filter(|(_, line)| line.iter().all(|c| *c == '.'))
            .map(|(row, _)| row)
            .collect();

        let mut empty_cols: HashSet<usize> = HashSet::from_iter(0..sky[0].len());
        sky.iter().for_each(|line| {
            line.iter().enumerate().for_each(|(col, char)| {
                if *char != '.' {
                    empty_cols.remove(&col);
                }
            })
        });

        let mut galaxies = Vec::new();
        let mut expanded_row = 0;
        for (row, line) in sky.iter().enumerate() {
            if empty_rows.contains(&row) {
                expanded_row += expansion - 1;
            }
            let mut expanded_col = 0;
            for (col, _) in line.iter().enumerate() {
                if empty_cols.contains(&col) {
                    expanded_col += expansion - 1;
                }
                if line[col] == '#' {
                    galaxies.push((expanded_row, expanded_col));
                }
                expanded_col += 1;
            }
            expanded_row += 1;
        }

        Self { galaxies }
    }
}
