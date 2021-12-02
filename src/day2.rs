use itertools::Itertools;

pub fn part_one(input: String) -> i32 {
    let mut x = 0;
    let mut y = 0;

    for chunk in input.split_whitespace().chunks(2).into_iter() {
        let line = chunk.collect::<Vec<&str>>();
        match (line[0], line[1].parse::<i32>().unwrap()) {
            ("forward", value) => x += value,
            ("down", value) => y += value,
            ("up", value) => y -= value,
            _ => (),
        }
    }

    x * y
}

pub fn part_two(input: String) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;

    for chunk in input.split_whitespace().chunks(2).into_iter() {
        let line = chunk.collect::<Vec<&str>>();
        match (line[0], line[1].parse::<i32>().unwrap()) {
            ("forward", val) => {
                x += val;
                y += aim * val;
            }
            ("down", val) => aim += val,
            ("up", val) => aim -= val,
            _ => (),
        }
    }

    x * y
}

#[test]
fn example_input_part1() {
    let input = include_str!("../day2sample.input");
    assert_eq!(part_one(input.to_string()), 150);
}

#[test]
fn example_input_part2() {
    let input = include_str!("../day2sample.input");
    assert_eq!(part_two(input.to_string()), 900);
}
