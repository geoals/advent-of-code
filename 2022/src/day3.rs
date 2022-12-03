use std::collections::HashSet;
use itertools::Itertools;

pub fn part_one(input: &str) -> i32 {
    input.lines().map(|line| {
        let first_set: HashSet<char> = (&line[0 .. line.len()/2]).chars().collect();
        let second_set: HashSet<char> = (&line[line.len()/2 .. line.len()]).chars().collect();
        let target = *first_set.intersection(&second_set).next().unwrap();
        priority(target)
    }).sum()
}

fn priority(c: char) -> i32 {
    if c.is_ascii_lowercase() { c as i32 - 96 } else { c as i32 - 38 }
}

pub fn part_two(input: &str) -> i32 {
    input.lines().chunks(3).into_iter().map(|mut group| {
        let first_set: HashSet<char> = group.next().unwrap().chars().collect();
        let second_set: HashSet<char> = group.next().unwrap().chars().collect();
        let third_set: HashSet<char> = group.next().unwrap().chars().collect();
        let intersection_set: HashSet<char> = first_set.intersection(&second_set).copied().collect();
        let target = *intersection_set.intersection(&third_set).next().unwrap();
        priority(target)
    }).sum()
}

#[test]
fn example_input_part1() {
    let input = include_str!("../input/sample_3.txt");
    assert_eq!(part_one(input), 157);
}

#[test]
fn example_input_part2() {
    let input = include_str!("../input/sample_3.txt");
    assert_eq!(part_two(input), 70);
}