use std::collections::{HashSet, VecDeque};
use itertools::Itertools;

pub fn part_one(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let start = find_pos(&grid, 'S');
    let end = find_pos(&grid, 'E');
    find_shortest_path(&grid, start, end)
}

pub fn part_two(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let end = find_pos(&grid, 'E');
    let start_positions = find_starting_positions_part2(&grid);
    start_positions.into_iter()
        .map(|pos| find_shortest_path(&grid, pos, end))
        .filter(|&x| x != 0)
        .min().unwrap()
}

type StepCount = usize;
type Pos = (usize, usize);

fn find_pos(grid: &[Vec<char>], char: char) -> Pos {
    let width = grid[0].len();
    let height = grid.len();
    (0..width).cartesian_product(0..height).find(|&(x, y)| {
        grid[y][x] == char
    }).unwrap()
}

fn find_starting_positions_part2(grid: &[Vec<char>]) -> Vec<Pos> {
    let width = grid[0].len();
    let height = grid.len();
    (0..width).cartesian_product(0..height).filter(|&(x, y)| {
        grid[y][x] == 'a' || grid[y][x] == 'S'
    }).collect()
}

fn find_shortest_path(grid: &[Vec<char>], start: Pos, end: Pos) -> StepCount {
    let mut visited = HashSet::<Pos>::new();
    let mut queue = VecDeque::<(Pos, StepCount)>::new();

    queue.push_back((start, 0));
    visited.insert(start);

    while !queue.is_empty() {
        let (current_pos, current_steps) = queue.pop_front().unwrap();
        let neighbors = get_unvisited_neighbors(grid, current_pos, &visited);

        for neighbor in neighbors {
            queue.push_back((neighbor, current_steps + 1));
            visited.insert(neighbor);
            if neighbor.0 == end.0 && neighbor.1 == end.1 {
                return current_steps + 1
            }
        }
    }

    0
}

fn get_unvisited_neighbors(grid: &[Vec<char>], position: Pos, visited: &HashSet::<Pos>) -> Vec<Pos> {
    let mut neighbors = vec![];
    let (x, y) = position;

    if x > 0 && is_valid_direction(grid, position, (x - 1, y)) && !visited.contains(&(x - 1, y)) {
        neighbors.push((x - 1, y))
    }
    if x < grid[0].len() - 1 && is_valid_direction(grid, position, (x + 1, y)) && !visited.contains(&(x + 1, y))  {
        neighbors.push((x + 1, y))
    }
    if y > 0 && is_valid_direction(grid, position, (x, y - 1)) && !visited.contains(&(x, y - 1))  {
        neighbors.push((x, y - 1))
    }
    if y < grid.len() - 1 && is_valid_direction(grid, position, (x, y + 1)) && !visited.contains(&(x, y + 1)) {
        neighbors.push((x, y + 1))
    }

    neighbors
}

fn is_valid_direction(grid: &[Vec<char>], from: Pos, to: Pos) -> bool {
    let from_char = grid[from.1][from.0];
    let from_char_i32 = if from_char == 'S' { 'a' } else { from_char } as i32;
    let to_char = grid[to.1][to.0];
    let to_char_i32 = if to_char == 'E' { 'z' } else { to_char } as i32;

    from_char_i32 >= to_char_i32 - 1
}

#[test]
fn test_get_neighbors() {
    let input = "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let expected = vec![(1, 0), (0, 1)];
    assert_eq!(expected, get_unvisited_neighbors(&grid, (0, 0), &HashSet::new()));

    let expected = vec![(1, 4), (0, 3)];
    assert_eq!(expected, get_unvisited_neighbors(&grid, (0, 4), &HashSet::new()));
}

#[test]
fn test_is_valid_direction() {
    let input = "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    assert_eq!(true, is_valid_direction(&grid, (1, 0), (2, 0)));
    assert_eq!(false, is_valid_direction(&grid, (2, 0), (3, 0)));
    assert_eq!(true, is_valid_direction(&grid, (1, 0), (1, 1)));
}

#[test]
fn test_part_one() {
    let input = "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
    assert_eq!(31, part_one(input));
}
#[test]

fn test_part_two() {
    let input = "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
    assert_eq!(29, part_two(input));
}
