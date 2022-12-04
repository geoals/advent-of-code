pub fn part_one(input: &str) -> i32 {
    input.lines().map(|line| {
        let splitline = line.split(',').collect::<Vec<&str>>();
        if is_fully_contained(splitline[0], splitline[1]) { 1 } else { 0 }
    }).sum()
}

fn is_fully_contained(a: &str, b: &str) -> bool {
    // let a_start = (a.as_bytes()[0] as char).to_digit(10).unwrap();
    // let a_end = (a.as_bytes()[2] as char).to_digit(10).unwrap();
    // let b_start = (b.as_bytes()[0] as char).to_digit(10).unwrap();
    // let b_end = (b.as_bytes()[2] as char).to_digit(10).unwrap();
    let a_split = a.split('-').map(|x| x.parse().unwrap()).collect::<Vec<i32>>();
    let b_split = b.split('-').map(|x| x.parse().unwrap()).collect::<Vec<i32>>();
    // println!("a: {}-{}, b: {}-{}", a_split[0], a_split[1], b_split[0], b_split[1]);
    // println!("{}", (a_split[0] <= b_split[0] && a_split[1] >= b_split[1]) || (b_split[0] <= a_split[0] && b_split[1] >= a_split[1]));
    println!("{}", (a_split[0] >= b_split[0] && a_split[1] <= b_split[1]) || (b_split[0] >= a_split[0] && b_split[1] <= a_split[1]));
    println!("is {} bigger or equal to {}?: {}", a_split[0], b_split[0], a_split[0] >= b_split[0]);
    println!("{}", a_split[1] <= b_split[1]);
    (a_split[0] >= b_split[0] && a_split[1] <= b_split[1]) || (b_split[0] >= a_split[0] && b_split[1] <= a_split[1])
    // (a_split[0] <= b_split[0] && a_split[1] >= b_split[1]) || (b_split[0] <= a_split[0] && b_split[1] >= a_split[1])
}



pub fn part_two(input: &str) -> i32 {
    1
}

#[test]
fn example_input_part1() {
    let input = include_str!("../input/sample_4.txt");
    assert_eq!(part_one(input), 3);
}

#[test]
fn example_input_part1_2() {
    let input = "\
53-62,8-87
";
    assert_eq!(part_one(input), 3);
}

#[test]
fn example_input_part2() {
    let input = include_str!("../input/sample_4.txt");
    assert_eq!(part_two(input), 0);
}
