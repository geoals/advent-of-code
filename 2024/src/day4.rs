pub fn part_one(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let reversed_grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().rev().collect()).collect();

    let mut num_occurences = 0;

    for row in input.lines() {
        num_occurences += row.matches("XMAS").count();
        num_occurences += row.matches("SAMX").count();
    }

    for row in transpose(input) {
        num_occurences += row.matches("XMAS").count();
        num_occurences += row.matches("SAMX").count();
    }

    num_occurences
        + find_diagonal_words(&grid, "XMAS").len()
        + find_diagonal_words(&grid, "SAMX").len()
        + find_diagonal_words(&reversed_grid, "XMAS").len()
        + find_diagonal_words(&reversed_grid, "SAMX").len()
}

fn transpose(input: &str) -> Vec<String> {
    let lines: Vec<&str> = input.lines().collect();
    let width = lines[0].len();
    let mut result = vec![String::with_capacity(lines.len()); width];

    for line in lines {
        for (i, ch) in line.chars().enumerate() {
            result[i].push(ch);
        }
    }

    result
}

fn find_diagonal_words(grid: &[Vec<char>], word: &str) -> Vec<(usize, usize)> {
    let rows = grid.len();
    let cols = grid[0].len();
    let word_chars: Vec<char> = word.chars().collect();
    let word_len = word_chars.len();
    let mut positions = Vec::new();

    for i in 0..rows {
        for j in 0..cols {
            if i + word_len <= rows && j + word_len <= cols {
                let mut matches = true;
                for k in 0..word_len {
                    if grid[i + k][j + k] != word_chars[k] {
                        matches = false;
                        break;
                    }
                }
                if matches {
                    positions.push((i, j));
                }
            }
        }
    }

    positions
}

pub fn part_two(input: &str) -> i64 {
    0
}

#[test]
fn test_part_one() {
    let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"#;

    assert_eq!(part_one(input), 18);
}
