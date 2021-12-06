use std::collections::HashMap;

fn into_i64_vec(input: &str) -> Vec<i64> {
    input.split(",").map(|n| n.parse().unwrap()).collect()
}

fn run_fish_simulation(input: &str, number_of_iterations: i64) -> usize {
    let mut numbers = into_i64_vec(input);

    for _ in 0..number_of_iterations {
        let mut new_numbers = vec![];
        for number in &mut numbers {
            *number -= 1;
            if *number < 0 {
                new_numbers.push(8);
                *number = 6;
            }
        }
        numbers.append(&mut new_numbers);
    }

    numbers.len()
}

pub fn part_one(input: &str) -> usize {
    run_fish_simulation(input, 80)
}

pub fn part_two(input: &str) -> i64 {
    // Values derived from bruteforcing a 256 iteration simulation with a '5' as input
    let values: HashMap<i64, i64> = HashMap::from([
        (1, 6206821033),
        (2, 5617089148),
        (3, 5217223242),
        (4, 4726100874),
        (5, 4368232009),
    ]);
    let numbers = into_i64_vec(input);
    let mut sum = 0;
    for n in numbers {
        sum += values[&n];
    }
    sum
}

#[test]
fn part1_example() {
    let input = "3,4,3,1,2";
    assert_eq!(run_fish_simulation(input, 80), 5934);
}