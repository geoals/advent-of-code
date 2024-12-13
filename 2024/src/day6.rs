use hashbrown::{HashMap, HashSet};

use rayon::prelude::*;

pub fn part_one(input: &str) -> usize {
    let map = build_map(input);
    let (position, direction) = start_pos_and_direction(&map);
    let visited = traverse_map(&map, position, direction).unwrap();
    visited.len()
}

pub fn part_two(input: &str) -> usize {
    let map = build_map(input);
    let (position, direction) = start_pos_and_direction(&map);
    let visited = traverse_map(&map, position, direction).unwrap();

    visited
        .par_iter()
        .map(|&((x, y), _)| {
            let mut new_map = map.clone();
            new_map[y][x] = '#';

            fast_traverse_map_and_detect_cycle(&new_map, position, direction) as usize
        })
        .sum()
}

/// Returns None if a cycle is detected
pub fn traverse_map(
    map: &[Vec<char>],
    mut position: (usize, usize),
    mut direction: char,
) -> Option<Vec<((usize, usize), char)>> {
    let mut visited: Vec<Vec<Option<char>>> = vec![vec![None; map[0].len()]; map.len()];
    visited[position.1][position.0] = Some(direction);

    while inside_bounds(map, &position, &direction) {
        if is_blocked(map, &position, &direction) {
            turn_right(&mut direction);
        }
        move_forward(&mut position, &direction);

        if let Some(visited_direction) = visited[position.1][position.0] {
            if direction == visited_direction {
                return None;
            }
        }

        visited[position.1][position.0] = Some(direction);
    }

    // counter instead
    // Transform full 2D grid to list of visited positions and direction tuple
    let mut positions = Vec::new();
    for (y, row) in visited.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if let Some(c) = cell {
                positions.push(((x, y), c));
            }
        }
    }

    Some(positions)
}

pub fn fast_traverse_map_and_detect_cycle(
    map: &[Vec<char>],
    mut position: (usize, usize),
    mut direction: char,
) -> bool {
    let mut visited_set = HashSet::new();
    visited_set.insert((position, direction));

    while inside_bounds(map, &position, &direction) {
        if is_blocked(map, &position, &direction) {
            turn_right(&mut direction);
        }

        let distance = distance_to_obstacle(map, &position, &direction);
        if distance == 0 {
            continue;
        }

        move_forward_by(distance, &mut position, &direction);

        if visited_set.contains(&(position, direction)) {
            return true;
        }
        visited_set.insert((position, direction));
    }

    false
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct DistanceKey {
    position: (usize, usize),
    direction: char,
}

/// Returns the distance to the next obstacle or boundary in the given direction
fn distance_to_obstacle(map: &[Vec<char>], position: &(usize, usize), direction: &char) -> usize {
    match direction {
        '<' => (0..position.0)
            .rev()
            .take_while(|&x| map[position.1][x] != '#')
            .count(),
        '>' => (position.0 + 1..map[0].len())
            .take_while(|&x| map[position.1][x] != '#')
            .count(),
        '^' => (0..position.1)
            .rev()
            .take_while(|&y| map[y][position.0] != '#')
            .count(),
        'V' => (position.1 + 1..map.len())
            .take_while(|&y| map[y][position.0] != '#')
            .count(),
        _ => panic!("Invalid direction"),
    }
}

fn build_map(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn start_pos_and_direction(input: &[Vec<char>]) -> ((usize, usize), char) {
    for (y, row) in input.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if ['<', '>', 'V', '^'].contains(tile) {
                return ((x, y), *tile);
            }
        }
    }
    panic!("No starting position found");
}

fn inside_bounds(map: &[Vec<char>], position: &(usize, usize), direction: &char) -> bool {
    match direction {
        '<' => position.0 > 0,
        '>' => position.0 < map[0].len() - 1,
        '^' => position.1 > 0,
        'V' => position.1 < map.len() - 1,
        _ => panic!("Invalid direction"),
    }
}

fn is_blocked(map: &[Vec<char>], position: &(usize, usize), direction: &char) -> bool {
    match direction {
        '<' => map[position.1][position.0 - 1] == '#',
        '>' => map[position.1][position.0 + 1] == '#',
        '^' => map[position.1 - 1][position.0] == '#',
        'V' => map[position.1 + 1][position.0] == '#',
        _ => panic!("Invalid direction"),
    }
}

fn turn_right(direction: &mut char) {
    match direction {
        '<' => *direction = '^',
        '^' => *direction = '>',
        '>' => *direction = 'V',
        'V' => *direction = '<',
        _ => panic!("Invalid direction"),
    }
}

fn move_forward(position: &mut (usize, usize), direction: &char) {
    match direction {
        '<' => position.0 -= 1,
        '>' => position.0 += 1,
        '^' => position.1 -= 1,
        'V' => position.1 += 1,
        _ => panic!("Invalid direction"),
    }
}

fn move_forward_by(distance: usize, position: &mut (usize, usize), direction: &char) {
    match direction {
        '<' => position.0 -= distance,
        '>' => position.0 += distance,
        '^' => position.1 -= distance,
        'V' => position.1 += distance,
        _ => panic!("Invalid direction"),
    }
}

#[test]
fn test_part_one() {
    let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

    assert_eq!(part_one(input), 41);
}

#[test]
fn test_part_two() {
    let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

    assert_eq!(part_two(input), 6);
}
