use std::collections::HashMap;

pub fn part_one(input: &str) -> i64 {
    let (mut list_one, mut list_two) = input
        .lines()
        .map(|line| {
            let mut iter = line.split_whitespace().map(|n| n.parse::<i64>().unwrap());
            (iter.next().unwrap(), iter.next().unwrap())
        })
        .collect::<(Vec<i64>, Vec<i64>)>();

    list_one.sort();
    list_two.sort();

    list_one
        .iter()
        .zip(list_two.iter())
        .map(|(a, b)| (a - b).abs())
        .sum::<i64>()
}

pub fn part_two(input: &str) -> i64 {
    let (list_one, list_two) = input
        .lines()
        .map(|line| {
            let mut iter = line.split_whitespace().map(|n| n.parse::<i64>().unwrap());
            (iter.next().unwrap(), iter.next().unwrap())
        })
        .collect::<(Vec<i64>, Vec<i64>)>();

    let list_two_counts = list_two.iter().fold(HashMap::new(), |mut acc, &n| {
        *acc.entry(n).or_insert(0) += 1;
        acc
    });

    list_one
        .iter()
        .map(|n| list_two_counts.get(n).unwrap_or(&0) * n)
        .sum::<i64>()
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
