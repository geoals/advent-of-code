
pub fn part_one(input: &str) -> i32 {
    input.lines().map(score_part1).sum()
}

pub fn part_two(input: &str) -> i32 {
    input.lines().map(score_part2).sum()
}

fn score_part1(line: &str) -> i32 {
    match line {
        "A X" => 4,
        "A Y" => 8,
        "A Z" => 3,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 7,
        "C Y" => 2,
        "C Z" => 6,
        _ => 0
    }
}

fn score_part2(line: &str) -> i32 {
    match line {
        "A X" => 3,
        "A Y" => 4,
        "A Z" => 8,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 2,
        "C Y" => 6,
        "C Z" => 7,
        _ => 0
    }
}

#[test]
fn example_input_part1() {
    let input = include_str!("../input/sample_2.txt");
    assert_eq!(part_one(input), 15);
}

#[test]
fn example_input_part2() {
    let input = include_str!("../input/sample_2.txt");
    assert_eq!(part_two(input), 12);
}