pub fn part_one(input: &str) -> i64 {
    sum_distances(input, 2)
}

pub fn part_two(input: &str) -> i64 {
    sum_distances(input, 1_000_000)
}

fn sum_distances(input: &str, multiplier: i64) -> i64 {
    let universe = input.lines()
        .map(|row| row.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let galaxies = get_galaxy_positions(&universe, multiplier);

    galaxies.iter().map(|galaxy| {
        galaxies.iter().map(|other_galaxy| distance(*galaxy, *other_galaxy))
            .sum::<i64>()
    }).sum::<i64>() / 2
}

fn distance(a: (i64, i64), b: (i64, i64)) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn get_galaxy_positions(universe: &[Vec<char>], multiplier: i64) -> Vec<(i64, i64)> {
    let empty_rows = get_empty_rows(universe);
    let empty_cols = get_empty_columns(universe);

    let mut positions = Vec::new();
    for (row, line) in universe.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            if *c != '#' { continue; }
            positions.push(
                (row as i64 + (empty_rows[row] * (multiplier - 1)),
                 col as i64 + (empty_cols[col] * (multiplier - 1)))
            );
        }
    }
    positions
}

fn get_empty_rows(universe: &[Vec<char>]) -> Vec<i64> {
    let mut is_empty = vec![true; universe.len()];
    for (row, line) in universe.iter().enumerate() {
        for &cell in line {
            if cell == '#' { is_empty[row] = false; }
        }
    }
    cumulative_counts(is_empty)
}


fn get_empty_columns(universe: &[Vec<char>]) -> Vec<i64> {
    let mut is_empty = vec![true; universe[0].len()];
    for row in universe {
        for (col, &cell) in row.iter().enumerate() {
            if cell == '#' { is_empty[col] = false; }
        }
    }
    cumulative_counts(is_empty)
}

/// Collect list of bools to cumulative counts, e.g. [true, false, true] -> [1, 1, 2]
fn cumulative_counts(is_empty: Vec<bool>) -> Vec<i64> {
    is_empty.iter().scan(0, |acc, &x| {
        *acc += usize::from(x) as i64;
        Some(*acc)
    }).collect()
}

mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        assert_eq!(part_one(input), 374);
    }
}