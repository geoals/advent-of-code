pub fn part_one(input: &str) -> i32 {
    input.lines().map(|line| {
        let splitline = line.split(',').collect::<Vec<&str>>();
        is_fully_contained(splitline[0], splitline[1]) as i32
    }).sum()
}

pub fn part_two(input: &str) -> i32 {
    input.lines().map(|line| {
        let splitline = line.split(',').collect::<Vec<&str>>();
        overlaps(splitline[0], splitline[1]) as i32
    }).sum()
}

fn is_fully_contained(a: &str, b: &str) -> bool {
    let a_split = a.split('-').map(|x| x.parse().unwrap()).collect::<Vec<i32>>();
    let b_split = b.split('-').map(|x| x.parse().unwrap()).collect::<Vec<i32>>();
    (a_split[0] >= b_split[0] && a_split[1] <= b_split[1]) || (b_split[0] >= a_split[0] && b_split[1] <= a_split[1])
}

fn overlaps(a: &str, b: &str) -> bool {
    let a_split = a.split('-').map(|x| x.parse().unwrap()).collect::<Vec<i32>>();
    let b_split = b.split('-').map(|x| x.parse().unwrap()).collect::<Vec<i32>>();
    (a_split[0] >= b_split[0] && a_split[0] <= b_split[1]) || (b_split[0] >= a_split[0] && b_split[0] <= a_split[1])
}

#[test]
fn example_input_part1() {
    let input = include_str!("../input/sample_4.txt");
    assert_eq!(part_one(input), 3);
}

#[test]
fn example_input_part2() {
    let input = include_str!("../input/sample_4.txt");
    assert_eq!(part_two(input), 4);
}
