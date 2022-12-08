pub fn part_one(input: &str) -> usize {
    let mut num_visible = 0;
    let all_rows = input.lines().collect::<Vec<&str>>();
    for (i_row, row) in all_rows.iter().enumerate() {
        for (i_col, col) in row.chars().enumerate() {
            num_visible += is_visible(i_row, row, i_col, &all_rows) as usize;
        }
    }
    num_visible
}

pub fn part_two(input: &str) -> usize {
    let mut scenic_scores = vec![];
    let all_rows = input.lines().collect::<Vec<&str>>();
    for (i_row, row) in all_rows.iter().enumerate() {
        for (i_col, col) in row.chars().enumerate() {
            scenic_scores.push(scenic_score(i_row, row, i_col, &all_rows))
        }
    }
    *scenic_scores.iter().max().unwrap()
}

fn is_visible(y: usize, row: &str, x: usize, all_rows: &Vec<&str>) -> bool {
    if y == 0 || y == all_rows.len() - 1 || x == 0 || x == row.len() - 1 {
        return true
    }
    let row_heights = row.chars().map(|x| x.to_digit(10).unwrap()).collect::<Vec<u32>>();
    let current_height = row_heights[x];
    let mut col_heights = vec![];
    for i in 0..all_rows.len() {
        let height = (all_rows[i].as_bytes()[x] as char).to_digit(10).unwrap();
        col_heights.push(height);
    }
    if row_heights[0..x].iter().filter(|tree_height| **tree_height >= current_height).count() == 0 {
        return true;
    }
    if row_heights[x+1..].iter().filter(|tree_height| **tree_height >= current_height).count() == 0 {
        return true;
    }
    if col_heights[0..y].iter().filter(|tree_height| **tree_height >= current_height).count() == 0 {
        return true;
    }
    if col_heights[y+1..].iter().filter(|tree_height| **tree_height >= current_height).count() == 0 {
        return true;
    }
    false
}

fn scenic_score(y: usize, row: &str, x: usize, all_rows: &Vec<&str>) -> usize {
    let row_heights = row.chars().map(|x| x.to_digit(10).unwrap()).collect::<Vec<u32>>();
    let current_height = row_heights[x];
    let mut col_heights = vec![];
    for i in 0..all_rows.len() {
        let height = (all_rows[i].as_bytes()[x] as char).to_digit(10).unwrap();
        col_heights.push(height);
    }

    let num_right = score_direction(current_height, &row_heights, (x+1..row_heights.len()).collect());
    let num_down = score_direction(current_height, &col_heights, (y+1..col_heights.len()).collect());
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

