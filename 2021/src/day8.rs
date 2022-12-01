pub fn part_one(input: &str) -> usize {
    input
        .lines()
        .flat_map(|it| it.split(" | ").nth(1).unwrap().split(' '))
        .filter(|it| [2, 4, 3, 7].contains(&it.len()))
        .count()
}

pub fn part_two(input: &str) -> usize {
    0
}


#[test]
fn example_part1() {
    assert_eq!(part_one(include_str!("day8sample.input")), 26)
}