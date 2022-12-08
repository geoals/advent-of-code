pub fn part_one(input: &str) -> usize {
    let mut num_visible = 0;
    let all_rows = input.lines().collect::<Vec<&str>>();
    for (y, row) in all_rows.iter().enumerate() {
        for (x, _) in row.chars().enumerate() {
            num_visible += is_visible(y, row, x, &all_rows) as usize;
        }
    }
    num_visible
}

pub fn part_two(input: &str) -> usize {
    let mut scenic_scores = vec![];
    let all_rows = input.lines().collect::<Vec<&str>>();
    for (y, row) in all_rows.iter().enumerate() {
        for (x, _) in row.chars().enumerate() {
            scenic_scores.push(scenic_score(y, row, x, &all_rows))
        }
    }
    *scenic_scores.iter().max().unwrap()
}

fn is_visible(y: usize, row: &str, x: usize, all_rows: &Vec<&str>) -> bool {
    let row_heights = row.chars().map(|x| x.to_digit(10).unwrap()).collect::<Vec<u32>>();
    let col_heights = column_vec_from_rows(x, all_rows);
    let current_height = row_heights[x];

    visible_from_direction(current_height, &row_heights[0..x])
        || visible_from_direction(current_height, &row_heights[x + 1..])
        || visible_from_direction(current_height, &col_heights[0..y])
        || visible_from_direction(current_height, &col_heights[y + 1..])
}

fn visible_from_direction(current_height: u32, it: &[u32]) -> bool {
    if it.iter().filter(|tree_height| **tree_height >= current_height).count() == 0 {
        return true;
    }
    false
}

fn column_vec_from_rows(x: usize, all_rows: &Vec<&str>) -> Vec<u32> {
    let mut col_heights = vec![];
    for i in 0..all_rows.len() {
        let height = (all_rows[i].as_bytes()[x] as char).to_digit(10).unwrap();
        col_heights.push(height);
    }
    col_heights
}

fn scenic_score(y: usize, row: &str, x: usize, all_rows: &Vec<&str>) -> usize {
    let row_heights = row.chars().map(|x| x.to_digit(10).unwrap()).collect::<Vec<u32>>();
    let col_heights = column_vec_from_rows(x, all_rows);
    let current_height = row_heights[x];

    let num_right = score_direction(current_height, &row_heights, (x + 1..row_heights.len()).collect());
    let num_down = score_direction(current_height, &col_heights, (y + 1..col_heights.len()).collect());
    let num_left = score_direction(current_height, &row_heights, (0..x).rev().collect());
    let num_up = score_direction(current_height, &col_heights, (0..y).rev().collect());

    num_right * num_down * num_left * num_up
}

fn score_direction(current_height: u32, tree_heights: &Vec<u32>, range: Vec<usize>) -> usize {
    let mut num_up = 0;
    for i in range {
        if tree_heights[i] < current_height {
            num_up += 1;
        }
        if tree_heights[i] >= current_height {
            num_up += 1;
            break;
        }
    }
    num_up
}

#[test]
fn example_input_part1() {
    let input = "\
30373
25512
65332
33549
35390";
    assert_eq!(part_one(input), 21);
}

#[test]
fn example_input_part2() {
    let input = "\
30373
25512
65332
33549
35390";
    assert_eq!(part_two(input), 8);
}

