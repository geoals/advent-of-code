use std::{env::args, fs::read_to_string};

mod day1;
mod day2;
mod day3;

fn main() {
    let day = args().nth(1).unwrap();

    let path = format!("input/input_{}.txt", day);
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
        _ => (),
    }
}
