use std::{env::args, fs::read_to_string};

mod day1;
mod day2;

fn main() {
    let day = args().skip(1).next().unwrap();

    let path = format!("day{}.input", day);
    let input = read_to_string(path).unwrap();

    let mut part1_result = 0;
    let mut part2_result = 0;

    match day.as_str() {
        "1" => {
            part1_result = day1::part_one(input.clone());
            part2_result = day1::part_two(input.clone());
        }
        "2" => {
            part1_result = day2::part_one(input.clone());
            part2_result = day2::part_two(input.clone());
        }
        _ => (),
    }

    println!("Part one: {}", part1_result);
    println!("Part two: {}", part2_result);
}
