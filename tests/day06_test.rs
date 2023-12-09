use rustvent::day06::Day;
use rustvent::solution::Solution;
use rustvent::utils::read_input;

#[test]
fn part1_example() {
    let input = read_input(6, true, 1).unwrap();
    assert_eq!(Day.part1(&input), Some(288));
}
#[test]
fn part1_challenge() {
    let input = read_input(6, false, 1).unwrap();
    assert_eq!(Day.part1(&input), Some(128700));
}

#[test]
fn part2_example() {
    let input = read_input(6, true, 2).unwrap();
    assert_eq!(Day.part2(&input), Some(71503));
}
#[test]
fn part2_challenge() {
    let input = read_input(6, false, 2).unwrap();
    assert_eq!(Day.part2(&input), Some(39594072));
}
