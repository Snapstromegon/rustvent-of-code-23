use rustvent::day04::Day;
use rustvent::solution::Solution;
use rustvent::utils::read_input;

#[test]
fn part1_example() {
    let input = read_input(4, true, 1);
    assert_eq!(Day.part1(&input), Some(13));
}
#[test]
fn part1_challenge() {
    let input = read_input(4, false, 1);
    assert_eq!(Day.part1(&input), Some(23235));
}

#[test]
fn part2_example() {
    let input = read_input(4, true, 2);
    assert_eq!(Day.part2(&input), Some(30));
}
#[test]
fn part2_challenge() {
    let input = read_input(4, false, 2);
    assert_eq!(Day.part2(&input), Some(5920640));
}
