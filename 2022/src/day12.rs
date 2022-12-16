use std::collections::{HashMap, HashSet, VecDeque};
use itertools::Itertools;

pub fn part_one(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let width = grid[0].len();
    let height = grid.len();
    // (0..width).cartesian_product(0..height).map(|(x, y)| {
    //     println!("{}, {}: {}", x, y, grid[y][x]);
    // }).count();

    println!("{:?}", find_shortest_path(&grid, (0, 0), (5, 2)));
    0
}

type Pos = (usize, usize);

fn find_shortest_path(grid: &[Vec<char>], start: Pos, end: Pos) -> Vec<Pos> {
    let mut visited = HashSet::<Pos>::new();
    let mut queue = VecDeque::<Pos>::new();
    let mut steps = 0;

    queue.push_back(start);
    visited.insert(start);

    while !queue.is_empty() {
        // println!("Queue: {:?}", queue);
        let current_pos = queue.pop_front().unwrap();
        steps += 1;
        println!("Current pos: {:?}, {}", current_pos, grid[current_pos.1][current_pos.0]);
        let neighbors = get_unvisited_neighbors(grid, current_pos, &visited);
        // println!("Neighbors: {:?}", neighbors);
        for neighbor in neighbors {
            queue.push_back(neighbor);
            visited.insert(neighbor);
            if neighbor.0 == end.0 && neighbor.1 == end.1 {
                println!("{}", steps);
                return queue.into_iter().collect();
            }
            println!("{}", steps);
        }
        // get neighbors
        // for neighbor of neighbors: if not already visited: visit it
    }

    queue.into_iter().collect()
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
    // println!("checking if {:?} to {:?} is valid", from, to);
    let from_char = grid[from.1][from.0];
    let from_char_i32 = if from_char == 'S' { 'a' } else { from_char } as i32;
    let to_char = grid[to.1][to.0];
    let to_char_i32 = if to_char == 'E' { 'z' } else { to_char } as i32;
    let is_valid = from_char_i32 >= to_char_i32 - 1; // TODO remove the first part of this
    // println!("from {} to {}: {}", from_char, to_char, is_valid);
    is_valid
}

pub fn part_two(_input: &str) -> usize {
    0
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
    assert_eq!(false, is_valid_direction(&grid, (1, 0), (0, 0)));
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