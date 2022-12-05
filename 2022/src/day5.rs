use std::collections::VecDeque;

use regex::Regex;

pub fn part_one(input: &str) -> String {
    let (crates, commands) = input.split_once("\n\n").unwrap();
    let capacity = (crates.split_once('\n').unwrap().0.len() + 1) / 4;
    let mut stacks: Vec<VecDeque<char>> = vec![VecDeque::new(); capacity]; 
    for row in crates.lines() {
        let chars = row.as_bytes();
        for (stack_num, x) in (1..row.len()-1).step_by(4).enumerate() {
            if (chars[x] as char).is_alphabetic() {
                stacks[stack_num].push_front(chars[x] as char)
            }
        }
    };

    let re = Regex::new(r"[a-z]*").unwrap();
    commands.lines().for_each(|command| {
        let command_numbers = &*re.replace_all(command, "");
        let mut parts = command_numbers.split_whitespace();
        let count = parts.next().unwrap().parse::<i32>().unwrap();
        let from = parts.next().unwrap().parse::<i32>().unwrap() - 1;
        let to = parts.next().unwrap().parse::<i32>().unwrap() - 1;
        for _ in 0..count {
            let _crate = stacks[from as usize].pop_back().unwrap();
            stacks[to as usize].push_back(_crate);
        }
    });
    stacks.iter().map(|stack| {
        let top_of_stack = stack.back().unwrap();
        top_of_stack
    }).collect::<String>()
}

pub fn part_two(input: &str) -> String {
    let (crates, commands) = input.split_once("\n\n").unwrap();
    let capacity = (crates.split_once('\n').unwrap().0.len() + 1) / 4;
    let mut stacks: Vec<VecDeque<char>> = vec![VecDeque::new(); capacity]; 
    for row in crates.lines() {
        let chars = row.as_bytes();
        for (stack_num, x) in (1..row.len()-1).step_by(4).enumerate() {
            if (chars[x] as char).is_alphabetic() {
                stacks[stack_num].push_front(chars[x] as char)
            }
        }
    };

    let re = Regex::new(r"[a-z]*").unwrap();
    commands.lines().for_each(|command| {
        let command_numbers = &*re.replace_all(command, "");
        let mut parts = command_numbers.split_whitespace();
        let count = parts.next().unwrap().parse::<i32>().unwrap();
        let from = parts.next().unwrap().parse::<i32>().unwrap() - 1;
        let to = parts.next().unwrap().parse::<i32>().unwrap() - 1;
        for i in 0..count {
            let _crate = stacks[from as usize].pop_back().unwrap();
            let to_stack_len = stacks[to as usize].len();
            stacks[to as usize].insert(to_stack_len - i as usize, _crate);
        }
    });
    stacks.iter().map(|stack| {
        let top_of_stack = stack.back().unwrap();
        top_of_stack
    }).collect::<String>()
}

#[test]
fn example_input_part1() {
    let input = include_str!("../input/sample_5.txt");
    assert_eq!(part_one(input), "CMZ");
}

#[test]
fn example_input_part2() {
    let input = include_str!("../input/sample_5.txt");
    assert_eq!(part_two(input), "MCD");
}
