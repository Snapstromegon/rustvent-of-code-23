use std::{fmt::Display, str::FromStr};

use crate::solution::Solution;

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<usize> {
        let map: Map = input.parse().unwrap();
        let distance_map = map.calc_distance_map(64);
        let reachable_plots = distance_map
            .iter()
            .flatten()
            .filter(|d| d.is_some())
            .filter(|d| d.unwrap() % 2 == 0)
            .count();
        Some(reachable_plots)
    }

    fn part2(&self, input: &str) -> Option<usize> {
        // let (steps, input) = input.split_once("\n\n").unwrap();
        let map: Map = input.parse().unwrap();
        let mut total_cells = 0;
        let map_size = map.tiles.len();

        let wanted_steps = 26501365;
        // let wanted_steps = 23;
        // let wanted_steps: usize = steps.parse().unwrap();

        if wanted_steps > map_size / 2 {
            total_cells += count_1s(&map, wanted_steps);
            println!("1s: {}", total_cells);
            total_cells += count_2s(&map, wanted_steps);
            println!("2s: {}", total_cells);
            total_cells += count_3s(&map, wanted_steps);
            println!("3s: {}", total_cells);
            total_cells += count_4s(&map, wanted_steps);
            println!("4s: {}", total_cells);
            total_cells += count_5s(&map, wanted_steps);
        } else {
            let res = map.get_distance_even_odd(wanted_steps);
            total_cells += if wanted_steps % 2 == 0 { res.0 } else { res.1 };
        }
        Some(total_cells)
    }
}

/*
    ....1....
    ...232...
    ..24542..
    .2455542.
    135555531
    .2455542.
    ..24542..
    ...232...
    ....1....
*/

fn count_1s(map: &Map, wanted_steps: usize) -> usize {
    println!("1s:");
    let map_size = map.tiles.len();
    let steps_tldr = ((wanted_steps - 1) - map_size / 2) % map_size;
    println!("steps tldr: {}", steps_tldr);
    let mut clone = map.clone();
    clone.start = (0, map_size / 2);
    let s_right = clone.get_distance_even_odd(steps_tldr);
    clone.start = (map_size / 2, 0);
    let s_up = clone.get_distance_even_odd(steps_tldr);
    clone.start = (map_size - 1, map_size / 2);
    let s_left = clone.get_distance_even_odd(steps_tldr);
    clone.start = (map_size / 2, map_size - 1);
    let s_down = clone.get_distance_even_odd(steps_tldr);
    if steps_tldr % 2 == 0 {
        s_right.0 + s_up.0 + s_left.0 + s_down.0
    } else {
        s_right.1 + s_up.1 + s_left.1 + s_down.1
    }
}

fn count_2s(map: &Map, wanted_steps: usize) -> usize {
    println!("2s:");
    let map_size = map.tiles.len();
    if wanted_steps > map_size {
        let steps_in_corner = (wanted_steps - 1 - map_size) % (map_size * 2);
        let mut clone = map.clone();
        clone.start = (0, 0);
        let s_br = clone.get_distance_even_odd(steps_in_corner);
        clone.start = (map_size - 1, 0);
        let s_bl = clone.get_distance_even_odd(steps_in_corner);
        clone.start = (0, map_size - 1);
        let s_tr = clone.get_distance_even_odd(steps_in_corner);
        clone.start = (map_size - 1, map_size - 1);
        let s_tl = clone.get_distance_even_odd(steps_in_corner);

        let s_corners = if steps_in_corner % 2 == 0 {
            s_br.0 + s_bl.0 + s_tr.0 + s_tl.0
        } else {
            s_br.1 + s_bl.1 + s_tr.1 + s_tl.1
        };

        let corner_count = (wanted_steps - 1) / map_size;
        println!("2 corner count: {}", corner_count);
        s_corners * corner_count
    } else {
        0
    }
}

fn count_3s(map: &Map, wanted_steps: usize) -> usize {
    println!("3s:");
    let map_size = map.tiles.len();
    let steps_tldr = wanted_steps % map_size;
    if steps_tldr > map_size / 2 && wanted_steps > map_size {
        let steps_tldr = steps_tldr + map_size / 2;
        let mut clone = map.clone();
        clone.start = (0, map_size / 2);
        let s_right = clone.get_distance_even_odd(steps_tldr);
        clone.start = (map_size / 2, 0);
        let s_up = clone.get_distance_even_odd(steps_tldr);
        clone.start = (map_size - 1, map_size / 2);
        let s_left = clone.get_distance_even_odd(steps_tldr);
        clone.start = (map_size / 2, map_size - 1);
        let s_down = clone.get_distance_even_odd(steps_tldr);

        if steps_tldr % 2 == 0 {
            s_right.0 + s_up.0 + s_left.0 + s_down.0
        } else {
            s_right.1 + s_up.1 + s_left.1 + s_down.1
        }
    } else {
        0
    }
}

fn count_4s(map: &Map, wanted_steps: usize) -> usize {
    println!("4s:");
    let map_size = map.tiles.len();
    let steps_tldr = (wanted_steps % map_size) - 1;

    if steps_tldr > map_size / 2 {
        let steps_tldr = steps_tldr + map_size / 2;
        let mut clone = map.clone();
        clone.start = (0, 0);
        let s_br = clone.get_distance_even_odd(steps_tldr);
        clone.start = ((map_size - 1) / 2, 0);
        let s_bl = clone.get_distance_even_odd(steps_tldr);
        clone.start = (map_size - 1, map_size - 1);
        let s_tl = clone.get_distance_even_odd(steps_tldr);
        clone.start = (0, map_size - 1);
        let s_tr = clone.get_distance_even_odd(steps_tldr);

        let edge_count = (wanted_steps - map_size / 2) / map_size - 1;
        println!("4 edge count: {}", edge_count);

        (if steps_tldr % 2 == 0 {
            s_br.0 + s_bl.0 + s_tl.0 + s_tr.0
        } else {
            s_br.1 + s_bl.1 + s_tl.1 + s_tr.1
        }) * edge_count
    } else {
        0
    }
}

fn count_5s(map: &Map, wanted_steps: usize) -> usize {
    println!("5s:");
    let map_size = map.tiles.len();
    let square_distance = wanted_steps / map_size;
    let number_of_full_even = number_of_squares(square_distance - (square_distance + 1) % 2);
    let number_of_full_odd = number_of_squares(square_distance - square_distance % 2);
    println!("number of full even: {}", number_of_full_even);
    println!("number of full odd: {}", number_of_full_odd);
    let (full_map_even, full_map_odd) = map.get_distance_even_odd(map_size);

    if wanted_steps % 2 == 0 {
        full_map_even * number_of_full_even + full_map_odd * number_of_full_odd
    } else {
        full_map_even * number_of_full_odd + full_map_odd * number_of_full_even
    }
}

fn number_of_squares(square_distance: usize) -> usize {
    if square_distance >= 2 {
        (square_distance - 1) * 4 + number_of_squares(square_distance - 2)
    } else {
        square_distance
    }
}

fn print_distances(distance_map: &[Vec<Option<usize>>], map: &Map) {
    for (y, row) in distance_map.iter().enumerate() {
        for (x, distance) in row.iter().enumerate() {
            print!(
                "{} ",
                match distance {
                    Some(d) => format!("{:02}", d),
                    None => match map.get((x, y)) {
                        Some(Tile::Plot) => "..".to_string(),
                        Some(Tile::Rock) => "##".to_string(),
                        Some(Tile::Start) => "SS".to_string(),
                        None => "??".to_string(),
                    },
                }
            );
        }
        println!();
    }
}

#[derive(Debug, Clone)]
struct Map {
    tiles: Vec<Vec<Tile>>,
    start: (usize, usize),
}

impl Map {
    fn get(&self, coords: (usize, usize)) -> Option<Tile> {
        self.tiles
            .get(coords.1 % self.tiles.len())
            .and_then(|row| row.get(coords.0 % self.tiles[0].len()))
            .copied()
    }

    fn get_possible_neighbors(&self, coords: (usize, usize)) -> Vec<(usize, usize)> {
        let (x, y) = coords;
        let mut neighbors = Vec::new();
        if y > 0
            && self
                .get((x, y - 1))
                .map(|tile| tile.is_walkable())
                .unwrap_or(false)
        {
            neighbors.push((x, y - 1));
        }
        if self
            .get((x, y + 1))
            .map(|tile| tile.is_walkable())
            .unwrap_or(false)
        {
            neighbors.push((x, y + 1));
        }
        if x > 0
            && self
                .get((x - 1, y))
                .map(|tile| tile.is_walkable())
                .unwrap_or(false)
        {
            neighbors.push((x - 1, y));
        }
        if self
            .get((x + 1, y))
            .map(|tile| tile.is_walkable())
            .unwrap_or(false)
        {
            neighbors.push((x + 1, y));
        }
        neighbors
    }

    fn calc_distance_map(&self, max_distance: usize) -> Vec<Vec<Option<usize>>> {
        let mut distance_map = vec![vec![None; self.tiles[0].len()]; self.tiles.len()];
        let mut queue = Vec::new();
        queue.push((self.start.0, self.start.1, 0));
        while let Some((x, y, distance)) = queue.pop() {
            if y < distance_map.len()
                && x < distance_map[0].len()
                && (distance_map[y][x].is_none() || Some(distance) < distance_map[y][x])
            {
                distance_map[y][x] = Some(distance);
                if distance < max_distance {
                    for (nx, ny) in self.get_possible_neighbors((x, y)) {
                        queue.push((nx, ny, distance + 1));
                    }
                }
            }
        }
        distance_map
    }

    pub fn get_distance_even_odd(&self, max_distance: usize) -> (usize, usize) {
        let distance_map = self.calc_distance_map(max_distance);
        println!("distance map:");
        print_distances(&distance_map, self);
        let res = distance_map
            .iter()
            .flatten()
            .flatten()
            .fold((0, 0), |acc, item| {
                if item % 2 == 0 {
                    (acc.0 + 1, acc.1)
                } else {
                    (acc.0, acc.1 + 1)
                }
            });
        println!("even: {}, odd: {}", res.0, res.1);
        res
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles: Vec<Vec<Tile>> = s
            .lines()
            .map(|line| line.chars().map(Tile::from).collect())
            .collect();
        let start = tiles
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter().enumerate().find_map(|(x, tile)| {
                    if *tile == Tile::Start {
                        Some((x, y))
                    } else {
                        None
                    }
                })
            })
            .unwrap();
        Ok(Map { tiles, start })
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.tiles {
            for tile in row {
                write!(
                    f,
                    "{}",
                    match tile {
                        Tile::Start => 'S',
                        Tile::Plot => '.',
                        Tile::Rock => '#',
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Start,
    Plot,
    Rock,
}

impl Tile {
    fn is_walkable(&self) -> bool {
        matches!(self, Self::Plot | Self::Start)
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Plot,
            '#' => Self::Rock,
            'S' => Self::Start,
            _ => panic!("Invalid tile: {}", c),
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
        let input = read_input(21, true, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(42));
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(21, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(3751));
    }

    #[test]
    #[ignore]
    fn test_part2_example() {
        let input = read_input(21, true, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(609453319569496));
    }
    #[test]
    #[ignore]
    fn test_part2_challenge() {
        let input = read_input(21, false, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(619407349431167));
    }
}
