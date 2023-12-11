use std::vec;

pub fn part_one(input: &str) -> i32 {
    input.lines()
        .map(extrapolate)
        .sum()
}

pub fn part_two(input: &str) -> i32 {
    input.lines()
        .map(extrapolate_part2)
        .sum()
}

fn extrapolate(line: &str) -> i32 {
    let numbers = line.split_whitespace().map(|n| n.parse().unwrap()).collect::<Vec<i32>>();
    let mut rows = vec![numbers.clone()];
    get_rows(&numbers, &mut rows);
    get_next_number(rows)
}

fn extrapolate_part2(line: &str) -> i32 {
    let numbers = line.split_whitespace().map(|n| n.parse().unwrap()).collect::<Vec<i32>>();
    let mut rows = vec![numbers.clone()];
    get_rows(&numbers, &mut rows);
    get_previous_number(rows)
}

fn get_rows(numbers: &Vec<i32>, rows: &mut Vec<Vec<i32>>) {
    let new_numbers = (1..numbers.len())
        .map(|i| numbers[i] - numbers[i-1])
        .collect::<Vec<i32>>();
    rows.push(new_numbers.clone());
    if new_numbers.iter().any(|&n| n != 0) {
        get_rows(&new_numbers, rows);
    }
}

fn get_next_number(rows: Vec<Vec<i32>>) -> i32 {
    let mut rows_rev = rows.clone().into_iter().rev().collect::<Vec<Vec<i32>>>();
    for i in 1..rows_rev.len() {
        let val_to_push = rows_rev[i-1].last().unwrap() + rows_rev[i].last().unwrap();
        rows_rev[i].push(val_to_push);
    }
    *rows_rev.last().unwrap().last().unwrap()
}

fn get_previous_number(rows: Vec<Vec<i32>>) -> i32 {
    let mut rows_rev = rows.clone().into_iter().rev().collect::<Vec<Vec<i32>>>();
    for i in 1..rows_rev.len() {
        let val_to_push = rows_rev[i][0] - rows_rev[i-1][0];
        rows_rev[i].insert(0, val_to_push);
    }
    rows_rev.last().unwrap()[0]
}

mod tests {
    const SAMPLE_INPUT: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
"#;

    #[test]
    fn example_input_part1() {
        assert_eq!(super::part_one(SAMPLE_INPUT), 114);
    }

    #[test]
    fn get_rows() {
        let mut rows = vec![vec![0, 3, 6, 9, 12, 15]];
        let expected_rows = vec![vec![0, 3, 6, 9, 12, 15], vec![3, 3, 3, 3, 3], vec![0, 0, 0, 0]];
        super::get_rows(&[0, 3, 6, 9, 12, 15].to_vec(), &mut rows);
        assert_eq!(rows, expected_rows);
    }

    #[test]
    fn get_next_number() {
        let rows = vec![vec![0, 3, 6, 9, 12, 15], vec![3, 3, 3, 3, 3], vec![0, 0, 0, 0]];
        assert_eq!(super::get_next_number(rows), 18);
    }

    #[test]
    fn extrapolate() {
        assert_eq!(super::extrapolate("0 3 6 9 12 15"), 18);
        assert_eq!(super::extrapolate("1 3 6 10 15 21"), 28);
        assert_eq!(super::extrapolate("10 13 16 21 30 45"), 68);
    }

    #[test]
    fn example_input_part2() {
        assert_eq!(super::part_two(SAMPLE_INPUT), 2);
    }
}