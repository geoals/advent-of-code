use itertools::Itertools;

pub fn part_one(input: &str) -> usize {
    let parsed_lines: Vec<(i64, Vec<i64>)> = input
        .lines()
        .map(|l| {
            let (sum, numbers) = l.split_once(": ").unwrap();
            (
                sum.parse::<i64>().unwrap(),
                numbers
                    .split_whitespace()
                    .map(|n| n.parse::<i64>().unwrap())
                    .collect(),
            )
        })
        .collect();

    let second = parsed_lines[1].clone();

    valid_line(second.0, second.1);

    0
}

pub fn part_two(input: &str) -> usize {
    todo!()
}

fn valid_line(sum: i64, numbers: Vec<i64>) -> bool {
    let operator_combinations: Vec<(i32, usize)> =
        (0..=1).cartesian_product(0..numbers.len() - 1).collect();

    for i in 0..2u32.pow((numbers.len() - 1) as u32) {
        let mut result = numbers[0];
        print!("{}", numbers[0]);

        for pos in 0..numbers.len() - 1 {
            // Use 0/1 directly to choose operator
            let idx = (i / 2u32.pow(pos as u32)) % 2;
            let op = if idx == 0 { '*' } else { '+' };

            let next_num = numbers[pos + 1];
            result = match op {
                '*' => result * next_num,
                '+' => result + next_num,
                _ => unreachable!(),
            };

            print!(" {} {}", op, next_num);
        }
        println!(" = {}", result);
    }

    false
}

#[test]
fn test_part_one() {
    let input = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

    assert_eq!(part_one(input), 3749);
}
