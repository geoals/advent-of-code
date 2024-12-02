use itertools::Itertools;

pub fn part_one(input: &str) -> usize {
    let levels: Vec<Vec<i64>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect();

    levels.iter().filter(|level| is_safe(level)).count()
}

pub fn part_two(input: &str) -> i64 {
    0
}

fn is_safe(level: &[i64]) -> bool {
    let all_increasing = level
        .iter()
        .tuple_windows::<(&i64, &i64)>()
        .all(|(a, b)| a - b <= 3 && a - b > -0);

    let all_decreasing = level
        .iter()
        .tuple_windows::<(&i64, &i64)>()
        .all(|(a, b)| b - a <= 3 && b - a > 0);

    all_increasing || all_decreasing
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
    let input = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
    assert_eq!(part_two(input), 31);
}
