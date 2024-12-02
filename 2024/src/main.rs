use std::{env::args, fs::read_to_string};

use took::Timer;

mod day1;

fn main() {
    let timer = Timer::new();
    let day = args().nth(1).unwrap();

    let path = format!("input/input_{}.txt", day);
    let input = read_to_string(path).unwrap();

    match day.as_str() {
        "1" => {
            println!("Part one: {}", day1::part_one(&input));
            println!("Part two: {}", day1::part_two(&input));
        }
        _ => (),
    }

    println!("Finished in {}", timer.took());
}
