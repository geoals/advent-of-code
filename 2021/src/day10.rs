use std::collections::HashMap;
use itertools::Itertools;

pub fn part_one(input: &str) -> i64 {
    sum(input.lines().filter_map(is_corrupted).collect())
}

fn sum(chars: Vec<char>) -> i64 {
    let scores = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    chars.iter().fold(0, |sum, c| sum + scores[c] )
}

fn is_corrupted(line: &str) -> Option<char> {
    let mut stack = vec![];
    for c in line.chars() {
        if ['[','(','{','<'].contains(&c) {
            stack.push(c);
            continue;
        } 
        if c != to_closing_char(stack.pop().unwrap()) {
            return Some(c);
        }
    }
    None
}

fn to_closing_char(c: char) -> char {
    match c { '[' => ']', '(' => ')', '{' => '}', '<' => '>', _ => '🙈' }
}

pub fn part_two(input: &str) -> i64 {
    let scores: Vec<i64> = input
        .lines()
        .filter(|line| is_corrupted(line).is_none())
        .map(|l| sum_part2(&autocomplete(l)))
        .sorted()
        .collect();
    scores[scores.len() / 2]
}

fn autocomplete(line: &str) -> Vec<char> {
    let mut stack = vec![];
    for c in line.chars() {
        if ['[','(','{','<'].contains(&c) {
            stack.push(c);
        } else { stack.pop(); } 
    }
    stack.into_iter().rev().map(to_closing_char).collect::<Vec<char>>()
}

fn sum_part2(chars: &[char]) -> i64 {
    let scores = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);
    chars.iter().fold(0, |sum, c| sum * 5 + scores[c])
}