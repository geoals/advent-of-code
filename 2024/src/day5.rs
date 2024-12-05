use std::{
    cmp::Ordering::{Greater, Less},
    collections::HashMap,
};

use itertools::Itertools;

pub fn part_one(input: &str) -> i64 {
    let (rules_str, updates_str) = input.split_once("\n\n").unwrap();
    let rule_map = build_rule_map(rules_str);

    updates_str
        .lines()
        .filter(|update| is_valid_update(update, &rule_map))
        .map(get_middle_number)
        .sum()
}

pub fn part_two(input: &str) -> i64 {
    let (rules_str, updates_str) = input.split_once("\n\n").unwrap();
    let rule_map = build_rule_map(rules_str);

    updates_str
        .lines()
        .filter(|update| !is_valid_update(update, &rule_map))
        .map(|update| {
            update
                .split(',')
                .sorted_by(|a, b| {
                    if rule_map.get(a).unwrap().contains(b) {
                        Greater
                    } else {
                        Less
                    }
                })
                .join(",")
        })
        .map(|update| get_middle_number(&update))
        .sum()
}

fn build_rule_map(rules_str: &str) -> HashMap<&str, Vec<&str>> {
    let mut rule_map: HashMap<&str, Vec<&str>> = HashMap::new();

    for rule in rules_str.lines() {
        let (value, key) = rule.split_once("|").unwrap();
        rule_map
            .entry(key)
            .and_modify(|el| el.push(value))
            .or_insert(vec![value]);
    }
    rule_map
}

fn is_valid_update(update: &str, rule_map: &HashMap<&str, Vec<&str>>) -> bool {
    let numbers = update.split(',').collect::<Vec<&str>>();
    let mut printed = vec![];
    for (i, number) in numbers.iter().enumerate() {
        let rest = &numbers[i..];
        let rules = rule_map.get(number).unwrap();
        if rules.iter().any(|el| rest.contains(el)) {
            return false;
        }
        printed.push(number);
    }
    true
}

fn get_middle_number(numbers: &str) -> i64 {
    let numbers_i64 = numbers
        .split(',')
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    numbers_i64[(numbers_i64.len() - 1) / 2]
}

mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

    // #[test]
    // fn test_part_one() {
    //     assert_eq!(part_one(SAMPLE_INPUT), 143);
    // }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(SAMPLE_INPUT), 123);
    }
}

// "75": [
//     "97",
// ],
// "13": [
//     "97",
//     "61",
//     "29",
//     "47",
//     "75",
//     "53",
// ],
// "53": [
//     "47",
//     "75",
//     "61",
//     "97",
// ],
// "29": [
//     "75",
//     "97",
//     "53",
//     "61",
//     "47",
// ],
// "61": [
//     "97",
//     "47",
//     "75",
// ],
// "47": [
//     "97",
//     "75",
// ],
