use regex::Regex;

pub fn part_one(input: &str) -> i64 {
    Regex::new(r"mul\((\d+),(\d+)\)")
        .unwrap()
        .captures_iter(input)
        .map(|cap| cap[1].parse::<i64>().unwrap() * cap[2].parse::<i64>().unwrap())
        .sum()
}

pub fn part_two(input: &str) -> usize {
    // input.split_inclusive()
    0
}
