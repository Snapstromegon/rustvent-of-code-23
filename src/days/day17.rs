use std::collections::{BinaryHeap, HashMap};

use crate::solution::Solution;

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<usize> {
        let grid = input
            .lines()
            .map(|s| {
                s.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect::<Vec<u8>>()
            })
            .collect::<Vec<_>>();
        Some(dijkstra(grid, 1, 3) as usize)
    }

    fn part2(&self, input: &str) -> Option<usize> {
        let grid = input
            .lines()
            .map(|s| {
                s.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect::<Vec<u8>>()
            })
            .collect::<Vec<_>>();
        Some(dijkstra(grid, 4, 10) as usize)
    }
}

fn dijkstra(grid: Vec<Vec<u8>>, minstep: isize, maxstep: isize) -> isize {
    let mut dists = HashMap::new();
    let goal = (grid.len() - 1, grid[0].len() - 1);
    let max_rows = grid.len();
    let max_cols = grid[0].len();
    let mut heap = BinaryHeap::from_iter([(0, (0, 0, (0, 0)))]);
    while let Some((cost, (row, col, dir))) = heap.pop() {
        if (row, col) == goal {
            return -cost;
        }
        if dists.get(&(row, col, dir)).is_some_and(|&c| -cost > c) {
            continue;
        }
        let valid_dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .filter(|&step_dir| dir != step_dir && dir != (-step_dir.0, -step_dir.1))
            .filter(|&step_dir| {
                let new_row = (row as isize + step_dir.0 * minstep) as usize;
                let new_col = (col as isize + step_dir.1 * minstep) as usize;
                new_row < max_rows && new_col < max_cols
            });
        for (d_row, d_col) in valid_dirs {
            let mut next_cost = -cost;
            for dist in 1..minstep {
                let new_row = (row as isize + d_row * dist) as usize;
                let new_col = (col as isize + d_col * dist) as usize;
                next_cost += (grid[new_row][new_col]) as isize;
            }
            for dist in minstep..=maxstep {
                let new_row = (row as isize + d_row * dist) as usize;
                let new_col = (col as isize + d_col * dist) as usize;
                if new_row >= max_rows || new_col >= max_cols {
                    continue;
                }
                next_cost += (grid[new_row][new_col]) as isize;
                let key = (new_row, new_col, (d_row, d_col));
                if next_cost < *dists.get(&key).unwrap_or(&isize::MAX) {
                    dists.insert(key, next_cost);
                    heap.push((-next_cost, key));
                }
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;
    use crate::utils::read_input;

    #[test]
    fn test_part1_example() {
        let input = read_input(17, true, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(102));
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(17, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(1013));
    }

    #[test]
    fn test_part2_example() {
        let input = read_input(17, true, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(94));
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(17, false, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(1215));
    }
}
