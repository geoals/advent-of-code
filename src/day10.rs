pub fn part_one(input: &str) -> i64 {
    let lines: Vec<&str> = input.lines().collect();
    let mut corrupted_chars = vec![];
    for line in lines {
        if let Some(c) = is_corrupt(line) {
            corrupted_chars.push(c);
            continue;
        }
    }
    sum(&corrupted_chars)
}

fn sum(chars: &[char]) -> i64 {
    chars.iter().fold(0, |sum, c| {
        sum + match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => 0,
        }
    })
}

fn is_corrupt(line: &str) -> Option<char> {
    let mut stack = vec![];
    for c in line.chars() {
        match c {
            '[' | '(' | '{' | '<' => stack.push(c),
            ']' | ')' | '}' | '>' => {
                let result = stack.pop();
                if result.is_none() || match_to_closing_char(result.unwrap()) != c {
                    return Some(c);
                }
            }
            _ => return Some(c),
        }
    }
    None
}

fn match_to_closing_char(c: char) -> char {
    match c {
        '[' => ']',
        '(' => ')',
        '{' => '}',
        '<' => '>',
        _ => '🙈',
    }
}

pub fn part_two(input: &str) -> i64 {
    0
}

#[test]
fn example_part1() {
    let input = include_str!("../day10sample.input");
    assert_eq!(part_one(input), 26397);
}

#[test]
fn simpler_example_part1() {
    let input = "[(]\n";
    assert_eq!(part_one(input), 57);
}
