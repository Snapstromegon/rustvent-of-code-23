use rustvent::day03::Day;
use rustvent::solution::Solution;
use rustvent::utils::read_input;

#[test]
fn part1_example() {
    let input = read_input(3, true, 1);
    assert_eq!(Day.part1(&input), Some(4361));
}
#[test]
fn part1_challenge() {
    let input = read_input(3, false, 1);
    assert_eq!(Day.part1(&input), Some(556367));
}

#[test]
fn part2_example() {
    let input = read_input(3, true, 2);
    assert_eq!(Day.part2(&input), Some(467835));
}
#[test]
fn part2_challenge() {
    let input = read_input(3, false, 2);
    assert_eq!(Day.part2(&input), Some(89471771));
}
