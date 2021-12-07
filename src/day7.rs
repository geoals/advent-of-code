use itertools::Itertools;

pub fn part_one(input: &str) -> i32 {
    let numbers: Vec<i32> = input.split(',').map(|n| n.parse().unwrap()).sorted().collect();
    let median = numbers[numbers.len() / 2];
    numbers.iter().map(|&n| (n - median).abs()).sum()
}

// Might be off by one
pub fn part_two(input: &str) -> i32 {
    let numbers: Vec<i32> = input.split(',').map(|n| n.parse().unwrap()).collect();
    let average = numbers.iter().sum::<i32>() / numbers.len() as i32;
    numbers.iter().map(|&n| calc_fuel_part2(n, average)).sum()
}

fn calc_fuel_part2(from: i32, to: i32) -> i32 {
    (0..=(from - to).abs()).sum()
}
