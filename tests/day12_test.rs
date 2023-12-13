use rustvent::day12::Day;
use rustvent::solution::Solution;
use rustvent::utils::read_input;

#[test]
fn part1_example() {
    let input = read_input(12, true, 1).unwrap();
    assert_eq!(Day.part1(&input), Some(21));
}

#[test]
fn part1_challenge() {
    let input = read_input(12, false, 1).unwrap();
    assert_eq!(Day.part1(&input), Some(7541));
}

#[test]
fn part2_example() {
    let input = read_input(12, true, 2).unwrap();
    assert_eq!(Day.part2(&input), Some(525152));
}

#[test]
fn part2_challenge() {
    let input = read_input(12, false, 2).unwrap();
    assert_eq!(Day.part2(&input), Some(17485169859432));
}
