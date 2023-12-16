pub fn part_one(input: &str) -> i32 {
    let universe = expand_universe(input);
    let galaxies = get_galaxy_positions(&universe);
    galaxies.iter().map(|galaxy| {
        galaxies.iter().map(|other_galaxy| distance(*galaxy, *other_galaxy))
            .sum::<i32>()
    }).sum::<i32>() / 2
}

fn expand_universe(input: &str) -> Vec<Vec<char>> {
    let universe = input.lines()
        .map(|row| row.chars().collect())
        .collect::<Vec<Vec<char>>>();
    expand_rows(transpose(expand_rows(universe)))
}

fn expand_rows(rows: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_rows = Vec::new();
    for row in &rows {
        new_rows.push(row.clone());
        if row.iter().all(|c| *c == '.') {
            new_rows.push(row.clone());
        }
    }
    new_rows
}

fn transpose(rows: Vec<Vec<char>>) -> Vec<Vec<char>> {
    (0..rows[0].len())
        .map(|col| {
            (0..rows.len())
                .map(|row| rows[row][col])
                .collect()
        }).collect()
}

fn get_galaxy_positions(universe: &[Vec<char>]) -> Vec<(i32, i32)> {
    let mut positions = Vec::new();
    for (row, line) in universe.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            if *c == '#' {
                positions.push((row as i32, col as i32));
            }
        }
    }
    positions
}

fn distance(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

pub fn part_two(input: &str) -> i32 {
    0
}

mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "...#......
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

    #[test]
    fn test_part_two() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        assert_eq!(part_two(input), 0);
    }
}