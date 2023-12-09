use rustvent::day08::Day;
use rustvent::solution::Solution;
use rustvent::utils::read_input;

#[test]
fn part1_example() {
    let input = read_input(8, true, 1).unwrap();
    assert_eq!(Day.part1(&input), Some(6));
}
#[test]
fn part1_challenge() {
    let input = read_input(8, false, 1).unwrap();
    assert_eq!(Day.part1(&input), Some(22357));
}

#[test]
fn part2_example() {
    let input = read_input(8, true, 2).unwrap();
    assert_eq!(Day.part2(&input), Some(6));
}
#[test]
fn part2_challenge() {
    let input = read_input(8, false, 2).unwrap();
    assert_eq!(Day.part2(&input), Some(10371555451871));
}
