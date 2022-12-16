use std::{env::args, fs::read_to_string};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
// mod day7;
mod day8;
mod day9;
mod day10;
mod day12;

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
        // "7" => {
        //     println!("Part one: {}", day7::part_one(&input));
        //     println!("Part two: {}", day7::part_two(&input));
        // }
        "8" => {
            println!("Part one: {}", day8::part_one(&input));
            println!("Part two: {}", day8::part_two(&input));
        }
        "9" => {
            println!("Part one: {}", day9::part_one(&input));
            println!("Part two: {}", day9::part_two(&input));
        }
        "10" => {
            println!("Part one: {}", day10::part_one(&input));
            println!("Part two:");
            day10::part_two(&input);
        },
        "12" => {
            println!("Part one: {}", day12::part_one(&input));
            println!("Part two: {}", day12::part_two(&input));
        }
        _ => (),
    }
}
