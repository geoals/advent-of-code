use std::collections::VecDeque;

use regex::Regex;

pub fn part_one(input: &str) -> String {
    let (mut stacks, commands) = parse_input(input);

    commands.into_iter().for_each(|(count, from, to)| {
        for _ in 0..count {
            let _crate = stacks[from].pop_back().unwrap();
            stacks[to].push_back(_crate);
        }
    });

    get_string_from_top_of_stacks(stacks)
}

pub fn part_two(input: &str) -> String {
    let (mut stacks, commands) = parse_input(input);

    commands.into_iter().for_each(|(count, from, to)| {
        for i in 0..count {
            let _crate = stacks[from].pop_back().unwrap();
            let to_stack_len = stacks[to].len();
            stacks[to].insert(to_stack_len - i as usize, _crate);
        }
    });

    get_string_from_top_of_stacks(stacks)
}

fn parse_input(input: &str) -> (Vec<VecDeque<char>>, Vec<(usize, usize, usize)>) {
    let (crates, commands) = input.split_once("\n\n").unwrap();
    (create_stacks(crates), parse_commands(commands))
}

fn create_stacks(crates: &str) -> Vec<VecDeque<char>> {
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
    stacks
}

fn parse_commands(commands: &str) -> Vec<(usize, usize, usize)> {
    let re = Regex::new(r"[a-z]*").unwrap();
    commands.lines().map(|command| {
        let command_numbers = &*re.replace_all(command, "");
        let mut parts = command_numbers.split_whitespace();
        let count = parts.next().unwrap().parse::<usize>().unwrap();
        let from = parts.next().unwrap().parse::<usize>().unwrap() - 1;
        let to = parts.next().unwrap().parse::<usize>().unwrap() - 1;
        (count, from, to)
    }).collect()
}

fn get_string_from_top_of_stacks(stacks: Vec<VecDeque<char>>) -> String {
    stacks.iter().map(|stack| {
        stack.back().unwrap()
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
