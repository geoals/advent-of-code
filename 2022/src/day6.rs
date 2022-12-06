use std::collections::{VecDeque, HashSet};

pub fn part_one(input: &str) -> usize {
    get_solution(input, 4)
}

pub fn part_two(input: &str) -> usize {
    get_solution(input, 14)
}

fn get_solution(input: &str, length: usize) -> usize {
    let mut window = input[..length].chars().collect::<VecDeque<_>>();
    for (i, char) in input[length..].chars().enumerate() {
        if HashSet::<_>::from_iter(window.clone()).len() == length {
            return i + length;
        }
        window.push_back(char);
        window.pop_front();
    }
    0
}

#[test]
fn example_input_part1() {
    let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    assert_eq!(part_one(input), 7);
    let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    assert_eq!(part_one(input), 5);
    let input = "nppdvjthqldpwncqszvftbrmjlhg";
    assert_eq!(part_one(input), 6);
    let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    assert_eq!(part_one(input), 10);
    let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    assert_eq!(part_one(input), 11);
}