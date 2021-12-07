use std::{env::args, fs::read_to_string};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() {
    let day = args().nth(1).unwrap();

    let path = format!("day{}.input", day);
    let input = read_to_string(path).unwrap();

    match day.as_str() {
        "1" => {
            println!("Part one: {}", day1::part_one(&input));
            println!("Part two: {}", day1::part_two(&input));
        }
        "2" => {
            println!("Part one: {}", day2::part_one(&input));
            println!("Part two: {}", day2::part_two(&input));
        }
        "3" => {
            println!("Part one: {}", day3::part_one(&input));
            println!("Part two: {}", day3::part_two(&input));
        }
        "4" => {
            println!("Part one: {}", day4::part_one(&input));
            println!("Part two: {}", day4::part_two(&input));
        }
        "5" => {
            println!("Part one: {}", day5::part_one(&input));
            println!("Part two: {}", day5::part_two(&input));
        }
        "6" => {
            println!("Part one: {}", day6::part_one(&input));
            println!("Part two: {}", day6::part_two(&input));
        }
        "7" => {
            println!("Part one: {}", day7::part_one(&input));
            println!("Part two: {}", day7::part_two(&input));
        }
        _ => (),
    }
}
