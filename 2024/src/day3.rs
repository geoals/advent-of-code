use regex::Regex;

pub fn part_one(input: &str) -> i64 {
    Regex::new(r"mul\((\d+),(\d+)\)")
        .unwrap()
        .captures_iter(input)
        .map(|cap| cap[1].parse::<i64>().unwrap() * cap[2].parse::<i64>().unwrap())
        .sum()
}

pub fn part_two(input: &str) -> i64 {
    let re = Regex::new(r"^mul\((\d+),(\d+)\)").unwrap();

    input
        .match_indices("mul")
        .filter(|&(pos, _)| {
            input[..pos].rfind("do()").unwrap_or(0) >= input[..pos].rfind("don't()").unwrap_or(0)
        })
        .filter_map(|(pos, _)| re.captures(&input[pos..]))
        .map(|expr| expr[1].parse::<i64>().unwrap() * expr[2].parse::<i64>().unwrap())
        .sum()
}

#[test]
fn test_part_two() {
    let input = r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;
    assert_eq!(part_two(input), 48);
}
