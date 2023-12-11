use rustvent::day11::Day;
use rustvent::solution::Solution;
use rustvent::utils::read_input;

#[test]
fn part1_example() {
    let input = read_input(11, true, 1).unwrap();
    assert_eq!(Day.part1(&input), Some(374));
}

#[test]
fn part1_challenge() {
    let input = read_input(11, false, 1).unwrap();
    assert_eq!(Day.part1(&input), Some(9648398));
}

#[test]
fn part2_example() {
    let input = read_input(11, true, 2).unwrap();
    assert_eq!(Day.part2(&input), Some(82000210));
}

#[test]
fn part2_challenge() {
    let input = read_input(11, false, 2).unwrap();
    assert_eq!(Day.part2(&input), Some(618800410814));
}
