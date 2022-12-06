use std::collections::VecDeque;

pub fn part_one(input: &str) -> usize {
    let mut window = VecDeque::<char>::new();
    for (i, char) in input.chars().enumerate() {
        println!("{} {}", i, char);
        window.push_back(char);
        if i > 3 { 
            if () { // make into set and chet len = 4
                return i + 1;
            }
            window.pop_front();
        }
    }
    1
}

pub fn part_two(input: &str) -> usize {
    0
}

#[test]
fn example_input_part1() {
    let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    assert_eq!(part_one(input), 7);
    // let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    // assert_eq!(part_one(input), 5);
    // let input = "nppdvjthqldpwncqszvftbrmjlhg";
    // assert_eq!(part_one(input), 6);
    // let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    // assert_eq!(part_one(input), 10);
    // let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    // assert_eq!(part_one(input), 11);
}