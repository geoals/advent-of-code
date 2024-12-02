use itertools::Itertools;

pub fn part_one(input: &str) -> usize {
    let levels = read_input_to_lists(input);
    count(levels, is_safe)
}

pub fn part_two(input: &str) -> usize {
    let levels = read_input_to_lists(input);
    count(levels, is_safe_part_two)
}

fn read_input_to_lists(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect()
}

fn count(levels: Vec<Vec<i64>>, predicate: impl Fn(&[i64]) -> bool) -> usize {
    levels.iter().filter(|level| predicate(level)).count()
}

fn is_valid_sequence(level: &[i64], get_diff: impl Fn(&i64, &i64) -> i64) -> bool {
    level
        .iter()
        .tuple_windows()
        .all(|(a, b)| (1..=3).contains(&get_diff(a, b)))
}

fn is_safe(level: &[i64]) -> bool {
    is_valid_sequence(level, |a, b| a - b) || is_valid_sequence(level, |a, b| b - a)
}

fn is_safe_part_two(level: &[i64]) -> bool {
    (0..level.len()).any(|i| {
        let mut level = level.to_vec();
        level.remove(i);
        is_safe(&level)
    })
}

#[test]
fn example_input_part1() {
    let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"#;
    assert_eq!(part_one(input), 2);
}

#[test]
fn example_input_part2() {
    let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"#;
    assert_eq!(part_two(input), 4);
}
