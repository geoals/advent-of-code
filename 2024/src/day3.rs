use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref MUL_REGEX: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
}

pub fn part_one(input: &str) -> i64 {
    MUL_REGEX
        .captures_iter(input)
        .map(|cap| cap[1].parse::<i64>().unwrap() * cap[2].parse::<i64>().unwrap())
        .sum()
}

pub fn part_two(input: &str) -> i64 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let on_off = Regex::new(r"do\(\)|don't\(\)").unwrap();
    let mut enabled = true;
    let mut sum = 0;
    for sequence in input.split_inclusive(&re) {
        let on_offs = on_off.find_iter(sequence);
        for mathc in on_offs {
            match mathc.as_str() {
                "do()" => enabled = true,
                "don't()" => enabled = false,
                _ => {}
            }
        }
        if !enabled {
            continue;
        }

        sum += part_one(sequence);
    }

    sum
}

#[test]
fn test_part_two() {
    let input = r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;
    assert_eq!(part_two(input), 48);
}
