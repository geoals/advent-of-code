use regex::Regex;

pub fn part_one(input: &str) -> i32 {
    let lines = input.split('\n').collect::<Vec<&str>>();
    let digit_regex = Regex::new(r"\d").unwrap();

    let mut sum = 0;

    for line in lines {
        let matches = digit_regex
            .find_iter(line)
            .map(|m| m.as_str().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let the_number = format!("{}{}", matches[0], matches[matches.len() - 1])
            .parse::<i32>()
            .unwrap();
        sum += the_number;
    }

    sum
}

pub fn part_two(input: &str) -> i32 {
    let lines = input.split('\n').collect::<Vec<&str>>();
    let digit_regex = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let reverse_regex = Regex::new(r"\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").unwrap();

    let mut sum = 0;

    for line in lines {
        let matches = matches_as_i32(&digit_regex, line);

        let line_reversed = line.chars().rev().collect::<String>();
        let matches_reversed = matches_as_i32(&reverse_regex, &line_reversed);

        let the_number = format!("{}{}", matches[0], matches_reversed[0])
            .parse::<i32>()
            .unwrap();
        sum += the_number;
    }

    sum
}

fn matches_as_i32(digit_regex: &Regex, line: &str) -> Vec<i32> {
    let matches = digit_regex
        .find_iter(line)
        .map(|m| match m.as_str().parse::<i32>() {
            Ok(digit) => digit,
            Err(_) => digit_as_str_to_i32(m.as_str()),
        })
        .collect::<Vec<i32>>();
    matches
}

fn digit_as_str_to_i32(digit: &str) -> i32 {
    match digit {
        "1" | "one" | "eno" => 1,
        "2" | "two" | "owt" => 2,
        "3" | "three" | "eerht" => 3,
        "4" | "four" | "ruof" => 4,
        "5" | "five" | "evif" => 5,
        "6" | "six" | "xis" => 6,
        "7" | "seven" | "neves" => 7,
        "8" | "eight" | "thgie" => 8,
        "9" | "nine" | "enin" => 9,
        _ => panic!("Unknown digit: {}", digit),
    }
}

#[test]
fn example_input_part1() {
    let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
    assert_eq!(part_one(input), 142);
}

#[test]
fn example_input_part2() {
    let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;
    assert_eq!(part_two(input), 281);
}
