pub fn part_one(input: String) -> i32 {
    let values: Vec<i32> = input.lines().map(|x| x.parse().unwrap()).collect();
    let mut increased = 0;
    let mut previous = 0;

    for value in values.iter().skip(1) {
        if *value > previous {
            increased += 1;
        }
        previous = *value;
    }

    increased
}

pub fn part_two(input: String) -> i32 {
    let values: Vec<i32> = input.lines().map(|x| x.parse().unwrap()).collect();
    const WINDOW_SIZE: usize = 3;
    let mut increased = 0;
    let mut previous = 0;

    for i in 0..=values.len() - WINDOW_SIZE {
        let sum = values[i..i + WINDOW_SIZE].iter().sum();
        if i == 0 {
            previous = sum;
            continue;
        }
        if sum > previous {
            increased += 1;
        }
        previous = sum;
    }

    increased
}

#[test]
fn example_input_part1() {
    let input = include_str!("../day1sample.input");
    assert_eq!(part_one(input.to_string()), 7);
}

#[test]
fn example_input_part2() {
    let input = include_str!("../day1sample.input");
    assert_eq!(part_two(input.to_string()), 5);
}