pub fn part_one(input: &str) -> i32 {
    calories_sum(input).into_iter().max().unwrap()
}

pub fn part_two(input: &str) -> i32 {
    let mut calories = calories_sum(input);
    calories.sort();
    calories.reverse();
    calories.iter().take(3).sum()
}

fn calories_sum(input: &str) -> Vec<i32> {
    input.split("\n\n")
        .map(|a| a.trim()
            .split('\n')
            .map(|x| x.parse::<i32>().unwrap()).sum()
        )
        .collect()
}

#[test]
fn example_input_part1() {
    let input = include_str!("../input/sample_1.txt");
    assert_eq!(part_one(input), 24000);
}

#[test]
fn example_input_part2() {
    let input = include_str!("../input/sample_1.txt");
    assert_eq!(part_two(input), 45000);
}