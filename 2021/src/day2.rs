pub fn part_one(input: &str) -> i32 {
    let mut x = 0;
    let mut y = 0;

    for line in input.split('\n').map(|x| x.split_once(' ').unwrap()) {
        let value = line.1.parse::<i32>().unwrap();
        match line.0 {
            "forward" => x += value,
            "down" => y += value,
            "up" => y -= value,
            _ => (),
        }
    }

    x * y
}

pub fn part_two(input: &str) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;

    for line in input.split('\n').map(|x| x.split_once(' ').unwrap()) {
        let val = line.1.parse::<i32>().unwrap();
        match line.0  {
            "forward" => {
                x += val;
                y += aim * val;
            }
            "down" => aim += val,
            "up" => aim -= val,
            _ => (),
        }
    }

    x * y
}

#[test]
fn example_input_part1() {
    let input = include_str!("../day2sample.input");
    assert_eq!(part_one(input), 150);
}

#[test]
fn example_input_part2() {
    let input = include_str!("../day2sample.input");
    assert_eq!(part_two(input), 900);
}
