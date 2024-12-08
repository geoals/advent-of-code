use std::collections::{HashMap, HashSet};

pub fn part_one(input: &str) -> usize {
    let grid = parse_to_vec_vec_char(input);
    let antennas = find_antennas_grouped_by_frequency(&grid);

    let mut all_antinodes = HashSet::new();

    for antennas in antennas.values() {
        let antinodes = find_all_antinodes(antennas);
        for antinode in antinodes {
            if in_bounds(antinode, &grid) {
                all_antinodes.insert(antinode);
            }
        }
    }

    all_antinodes.len()
}

pub fn part_two(input: &str) -> usize {
    let grid = parse_to_vec_vec_char(input);
    let antennas = find_antennas_grouped_by_frequency(&grid);

    let mut all_antinodes = HashSet::new();

    for antennas in antennas.values() {
        let antinodes =
            find_all_antinodes_part_two(antennas, grid[0].len() as i64, grid.len() as i64);
        for antinode in antinodes {
            if in_bounds(antinode, &grid) {
                all_antinodes.insert(antinode);
            }
        }
    }

    all_antinodes.len()
}

fn in_bounds(point: (i64, i64), grid: &[Vec<char>]) -> bool {
    point.0 >= 0 && point.1 >= 0 && point.0 < grid[0].len() as i64 && point.1 < grid.len() as i64
}

fn find_all_antinodes(antennas: &[(i64, i64)]) -> Vec<(i64, i64)> {
    let mut antinodes = vec![];
    for i in 0..antennas.len() {
        for j in i + 1..antennas.len() {
            let (a, b) = find_antinodes(antennas[i], antennas[j]);
            antinodes.push(a);
            antinodes.push(b);
        }
    }
    antinodes
}

fn find_all_antinodes_part_two(
    antennas: &[(i64, i64)],
    width: i64,
    height: i64,
) -> Vec<(i64, i64)> {
    let mut antinodes = vec![];
    for i in 0..antennas.len() {
        for j in i + 1..antennas.len() {
            let new = find_antinodes_part_two(antennas[i], antennas[j], width, height);
            antinodes.extend(new);
        }
    }
    antinodes
}

fn find_antinodes(a: (i64, i64), b: (i64, i64)) -> ((i64, i64), (i64, i64)) {
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;
    let first_antinode = (a.0 - dx, a.1 - dy);
    let second_antinode = (b.0 + dx, b.1 + dy);

    (first_antinode, second_antinode)
}

// all positions at dx, dy intervals out from a and b in a straight line within bounds are antinodes
fn find_antinodes_part_two(
    a: (i64, i64),
    b: (i64, i64),
    width: i64,
    height: i64,
) -> Vec<(i64, i64)> {
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;

    let mut x = a.0;
    let mut y = a.1;

    let mut antinodes = vec![];
    while x < width && y < height {
        antinodes.push((x, y));
        x += dx;
        y += dy;
    }

    x = a.0 - dx;
    y = a.1 - dy;
    while x >= 0 && y >= 0 {
        antinodes.push((x, y));
        x -= dx;
        y -= dy;
    }

    antinodes
}

fn parse_to_vec_vec_char(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn find_antennas_grouped_by_frequency(grid: &[Vec<char>]) -> HashMap<char, Vec<(i64, i64)>> {
    let mut antennas = HashMap::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == '.' {
                continue;
            }
            let entry = antennas.entry(cell).or_insert(vec![]);
            entry.push((x as i64, y as i64));
        }
    }
    antennas
}

#[test]
fn test_three_antennas() {
    let input = r#"..........
..........
..........
....a.....
........a.
.....a....
..........
..........
..........
.........."#;

    assert_eq!(part_one(input), 4);
}

#[test]
fn test_antinodes() {
    let a = (2, 2);
    let b = (3, 4);
    let (first_antinode, second_antinode) = find_antinodes(a, b);
    assert_eq!(first_antinode, (1, 0));
    assert_eq!(second_antinode, (4, 6));
}

#[test]
fn find_antennas_grouped_by_frequency_test() {
    let input = r#"
......
......
..a...
......
...a..
......
......"#;

    let expected_antenna_locations: HashMap<char, Vec<(i64, i64)>> =
        HashMap::from([('a', vec![(2, 2), (3, 4)])]);

    assert_eq!(
        expected_antenna_locations,
        find_antennas_grouped_by_frequency(&parse_to_vec_vec_char(input)),
    );
}

#[test]
fn test_part_two_three_nodes() {
    let input = r#"T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
.........."#;

    assert_eq!(part_two(input), 9);
}

#[test]
fn test_find_antinodes_part_two() {}
