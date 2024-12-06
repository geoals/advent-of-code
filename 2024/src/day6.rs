use std::{collections::HashSet, str::FromStr};

use Direction::*;
use Tile::*;

pub fn part_one(input: &str) -> usize {
    let map: Vec<Vec<Tile>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Tile::Obstacle,
                    '.' | '<' | '>' | '^' | 'V' => Tile::Traversable,
                    _ => panic!("Invalid character"),
                })
                .collect()
        })
        .collect();

    let mut starting_position = (0, 0);

    for (y, row) in input.lines().enumerate() {
        for (x, tile) in row.chars().enumerate() {
            if ['<', '>', 'V', '^'].contains(&tile) {
                starting_position = (x, y);
                break;
            }
        }
    }

    let mut guard = Guard {
        x: starting_position.0 as i64,
        y: starting_position.1 as i64,
        direction: Direction::from_str(
            &input
                .chars()
                .find(|c| ['<', '>', 'V', '^'].contains(c))
                .unwrap()
                .to_string(),
        )
        .unwrap(),
        finished_moving: false,
        visited: HashSet::from([(starting_position.1 as i64, starting_position.0 as i64)]),
    };

    while !guard.finished_moving {
        guard.move_one_tile(&map);
    }

    guard.visited.len()
}

pub fn part_two(input: &str) -> i64 {
    0
}

fn out_of_bounds(x: i64, y: i64, map: &[Vec<Tile>]) -> bool {
    x >= map[0].len() as i64 || y >= map.len() as i64 || x < 0 || y < 0
}

#[derive(Debug)]
struct Guard {
    x: i64,
    y: i64,
    direction: Direction,
    visited: HashSet<(i64, i64)>,
    finished_moving: bool,
}

impl Guard {
    fn move_one_tile(&mut self, map: &[Vec<Tile>]) {
        if self.is_blocked(map) {
            self.turn_right();
        }
        match self.direction {
            Left => self.x -= 1,
            Right => self.x += 1,
            Down => self.y += 1,
            Up => self.y -= 1,
        }

        if out_of_bounds(self.x, self.y, map) {
            self.finished_moving = true;
            return;
        }

        self.visited.insert((self.y, self.x));
    }

    // blocked if tile in front is not traversable
    fn is_blocked(&self, map: &[Vec<Tile>]) -> bool {
        let next_x = match self.direction {
            Left => {
                if self.x == 0 {
                    return false;
                }
                self.x - 1
            }
            Right => {
                if self.x as usize >= map[0].len() - 1 {
                    return false;
                }
                self.x + 1
            }
            _ => self.x,
        };

        let next_y = match self.direction {
            Up => {
                if self.y == 0 {
                    return false;
                }
                self.y - 1
            }
            Down => {
                if self.y as usize >= map.len() - 1 {
                    return false;
                }
                self.y + 1
            }
            _ => self.y,
        };

        map[next_y as usize][next_x as usize] == Obstacle
    }

    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Left => Up,
            Right => Down,
            Down => Left,
            Up => Right,
        }
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Down,
    Up,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Tile {
    Obstacle,
    Traversable,
}

impl FromStr for Direction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "<" => Ok(Left),
            ">" => Ok(Right),
            "V" => Ok(Down),
            "^" => Ok(Up),
            _ => Err(()),
        }
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
