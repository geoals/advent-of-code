use std::collections::HashSet;
use itertools::Itertools;

pub fn part_one(input: &str) -> i32 {
    input.lines().map(|line| {
        let left  = as_set(&line[..line.len() / 2]);
        let right = as_set(&line[line.len() / 2..]);
        *left.intersection(&right).next().unwrap()
    }).map(priority).sum()
}

pub fn part_two(input: &str) -> i32 {
    input.lines().tuples().map(|(a, b, c)| {
        let intersection_set: HashSet<char> = as_set(a).intersection(&as_set(b)).copied().collect();
        *intersection_set.intersection(&as_set(c)).next().unwrap()
    })
    .map(priority).sum()
}

fn priority(c: char) -> i32 {
    if c.is_ascii_lowercase() { c as i32 - 96 } else { c as i32 - 38 }
}

fn as_set(val: &str) -> HashSet<char> { 
    val.chars().collect() 
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