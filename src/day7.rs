pub fn part_one(input: &str) -> i32 {
    let mut numbers = into_i32_vec(input);
    part_1_median(&mut numbers)
}

pub fn part_two(input: &str) -> i32 {
    let numbers = into_i32_vec(input);
    part_2_average_floored(&numbers)
}

fn into_i32_vec(input: &str) -> Vec<i32> {
    input.split(',').map(|n| n.parse().unwrap()).collect()
}

fn part_1_median(numbers: &mut [i32]) -> i32 {
    numbers.sort();
    let median = numbers[numbers.len() / 2];
    numbers.iter().map(|&n| (n - median).abs()).sum()
}

// Might be off by one
fn part_2_average_floored(numbers: &[i32]) -> i32 {
    let average = numbers.iter().sum::<i32>() / numbers.len() as i32;
    numbers.iter().map(|&n| calc_fuel_part2(n, average)).sum()
}

fn calc_fuel_part2(from: i32, to: i32) -> i32 {
    (0..=(from - to).abs()).sum()
}