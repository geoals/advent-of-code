#![allow(dead_code, unused_variables, unused_mut)]

use std::collections::HashSet;
use std::ops::Range;

pub fn part_one(input: &str) -> i32 {
    input.lines().map(score_card).sum()
}

fn score_card(line: &str) -> i32 {
    match amount_of_correct_numbers(line) {
        0 => 0,
        n => 2_i32.pow(n - 1),
    }
}

fn amount_of_correct_numbers(line: &str) -> u32 {
    let (left, right) = line.split_once(" | ").unwrap();
    let winning_numbers: HashSet<i32> = left
        .split_once(": ").unwrap().1
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    let your_numbers: HashSet<i32> = right
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    your_numbers.intersection(&winning_numbers).count() as u32
}

pub fn part_two(input: &str) -> i32 {
    let cards = input.lines().collect::<Vec<&str>>();
    let mut card_numbers: Vec<usize> = (0..cards.len()).collect();
    let winning_numbers = get_winning_numbers_for_each_card(cards);

    let mut sum = 0;
    while let Some(card_number) = card_numbers.pop() {
        card_numbers.extend(winning_numbers[card_number].clone());
        sum += 1;
    }
    sum
}

fn get_winning_numbers_for_each_card(cards: Vec<&str>) -> Vec<Range<usize>> {
    cards.iter().enumerate().map(|(i, card)| {
        i + 1..(i + 1 + amount_of_correct_numbers(card) as usize)
    }).collect()
}

mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"#;

    #[test]
    fn example_input_part1() {
        assert_eq!(part_one(SAMPLE_INPUT), 13);
    }

    #[test]
    fn example_input_part2() {
        assert_eq!(part_two(SAMPLE_INPUT), 30);
    }
}
