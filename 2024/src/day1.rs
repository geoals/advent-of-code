use std::collections::HashMap;

pub fn part_one(input: &str) -> i64 {
    let (mut list_one, mut list_two) = parse_lines_to_i64_lists(input);
    list_one.sort();
    list_two.sort();

    list_one
        .iter()
        .zip(list_two.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

pub fn part_two(input: &str) -> i64 {
    let (list_one, list_two) = parse_lines_to_i64_lists(input);

    let counts = list_two.iter().fold(HashMap::new(), |mut acc, &n| {
        *acc.entry(n).or_insert(0) += 1;
        acc
    });

    list_one
        .iter()
        .map(|n| n * counts.get(n).unwrap_or(&0))
        .sum()
}

fn parse_lines_to_i64_lists(input: &str) -> (Vec<i64>, Vec<i64>) {
    input
        .lines()
        .map(|line| {
            let mut numbers = line.split_whitespace().map(|n| n.parse::<i64>().unwrap());
            (numbers.next().unwrap(), numbers.next().unwrap())
        })
        .unzip()
}

#[test]
fn example_input_part1() {
    let input = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
    assert_eq!(part_one(input), 11);
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
