use std::collections::{HashMap, HashSet};

pub fn part_one(input: &str) -> i32 {
    let pipe_grid: Vec<Vec<char>> = input.split('\n')
        .map(|line| line.chars().collect())
        .collect();

    let main_loop = find_main_loop(&pipe_grid);

    main_loop.len() as i32 / 2
}

fn find_main_loop(pipe_grid: &[Vec<char>]) -> HashMap<(usize, usize), char> {
    let starting_position = find_starting_position(pipe_grid);

    let mut traversed_tiles: HashMap<(usize, usize), char> = HashMap::new();

    let mut current_position = starting_position;
    loop {
        traversed_tiles.insert(current_position, pipe_grid[current_position.1][current_position.0]);

        match pipe_grid[current_position.1][current_position.0] {
            'S' => current_position = find_next_position(&[Direction::Down, Direction::Right, Direction::Left, Direction::Up], &traversed_tiles, current_position),
            'F' => current_position = find_next_position(&[Direction::Down, Direction::Right], &traversed_tiles, current_position),
            'L' => current_position = find_next_position(&[Direction::Up, Direction::Right], &traversed_tiles, current_position),
            'J' => current_position = find_next_position(&[Direction::Up, Direction::Left], &traversed_tiles, current_position),
            '7' => current_position = find_next_position(&[Direction::Down, Direction::Left], &traversed_tiles, current_position),
            '|' => current_position = find_next_position(&[Direction::Down, Direction::Up], &traversed_tiles, current_position),
            '-' => current_position = find_next_position(&[Direction::Left, Direction::Right], &traversed_tiles, current_position),
            _ => {}
        }

        if current_position == starting_position {
            break;
        }
    }

    traversed_tiles
}

fn find_next_position(
    directions: &[Direction],
    traversed_tiles: &HashMap<(usize, usize), char>,
    current_position: (usize, usize),
) -> (usize, usize) {
    for direction in directions {
        let new_position = move_to(direction, current_position);
        if !traversed_tiles.contains_key(&new_position) || traversed_tiles[&new_position] == 'S' {
            return new_position;
        }
    }
    panic!("No new position found");
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn move_to(direction: &Direction, current_position: (usize, usize)) -> (usize, usize) {
    match direction {
        Direction::Up => (current_position.0, current_position.1 - 1),
        Direction::Down => (current_position.0, current_position.1 + 1),
        Direction::Left => (current_position.0 - 1, current_position.1),
        Direction::Right => (current_position.0 + 1, current_position.1),
    }
}

fn find_starting_position(pipe_grid: &[Vec<char>]) -> (usize, usize) {
    for (y, row) in pipe_grid.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if *tile == 'S' {
                return (x, y);
            }
        }
    }
    panic!("No starting position found");
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Side { Above, Below, Left, Right, None }

// DFS from tile until x or y pos is at the edge of the grid
// return false if there is a path to the edge
fn is_inside_the_loop(tile: (usize, usize), pipe_grid: &[Vec<char>], main_loop: &HashMap<(usize, usize), char>) -> bool {
    if main_loop.contains_key(&tile) {
        return false;
    }

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut stack: Vec<(usize, usize)> = vec![tile];
    let mut side_stack: Vec<Side> = vec![Side::None];

    while let Some(current_pos) = stack.pop() {
        let side = side_stack.pop().unwrap();
        if current_pos.0 == 0 || current_pos.0 == pipe_grid[0].len() - 1 || current_pos.1 == 0 || current_pos.1 == pipe_grid.len() - 1 {
            return false;
        }

        visited.insert(current_pos);

        for direction in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right
        ].iter() {
            let next_pos = move_to(direction, current_pos);
            if visited.contains(&next_pos) {
                continue;
            }
            if !main_loop.contains_key(&next_pos) && !main_loop.contains_key(&current_pos) {
                // normal traversal not on or through main loop
                side_stack.push(Side::None);
                stack.push(next_pos);
            } else {
                let next_tile = pipe_grid[next_pos.1][next_pos.0];

                let is_valid_traversal_on_main_loop = main_loop.contains_key(&next_pos)
                    && match pipe_grid[current_pos.1][current_pos.0] {
                    '|' => (*direction == Direction::Up && (next_tile == '|' || next_tile == 'F' || next_tile == '7'))
                        || (*direction == Direction::Down && (next_tile == '|' || next_tile == 'J' || next_tile == 'L')),
                    '-' => (*direction == Direction::Left && (next_tile == '-' || next_tile == 'F' || next_tile == 'L'))
                        || (*direction == Direction::Right && (next_tile == '-' || next_tile == 'J' || next_tile == '7')),
                    'J' => (*direction == Direction::Up && (next_tile == '|' || next_tile == '7' || next_tile == 'F'))
                        || (*direction == Direction::Left && (next_tile == '-' || next_tile == 'L' || next_tile == 'F')),
                    'L' => (*direction == Direction::Up && (next_tile == '|' || next_tile == '7' || next_tile == 'F'))
                        || (*direction == Direction::Right && (next_tile == '-' || next_tile == 'J' || next_tile == '7')),
                    'F' => (*direction == Direction::Down && (next_tile == '|' || next_tile == 'J' || next_tile == 'L'))
                        || (*direction == Direction::Right && (next_tile == '-' || next_tile == 'J' || next_tile == '7')),
                    '7' => (*direction == Direction::Down && (next_tile == '|' || next_tile == 'J' || next_tile == 'L'))
                        || (*direction == Direction::Left && (next_tile == '-' || next_tile == 'L' || next_tile == 'F')),
                    _ => false,
                };

                if is_valid_traversal_on_main_loop {
                    side_stack.push(get_next_side(side, pipe_grid[current_pos.1][current_pos.0], next_tile));
                    stack.push(next_pos);
                }

                // traversal into main loop
                if main_loop.contains_key(&next_pos) && !main_loop.contains_key(&current_pos) {
                    if *direction == Direction::Down && (next_tile == 'F' || next_tile == '7') {
                        side_stack.push(Side::Above);
                        stack.push(next_pos);
                    }
                    if *direction == Direction::Up && (next_tile == 'J' || next_tile == 'L') {
                        side_stack.push(Side::Below);
                        stack.push(next_pos);
                    }
                    if *direction == Direction::Left && (next_tile == '7' || next_tile == 'J') {
                        if next_tile == '7' {
                            side_stack.push(Side::Above);
                        }
                        if next_tile == 'J' {
                            side_stack.push(Side::Below);
                        }
                        stack.push(next_pos);
                    }
                    if *direction == Direction::Right && (next_tile == 'L' || next_tile == 'F') {
                        if next_tile == 'L' {
                            side_stack.push(Side::Below);
                        }
                        if next_tile == 'F' {
                            side_stack.push(Side::Above);
                        }
                        stack.push(next_pos);
                    }
                }

                let is_valid_traversal_out_of_main_loop = !main_loop.contains_key(&next_pos) && main_loop.contains_key(&current_pos)
                    && match pipe_grid[current_pos.1][current_pos.0] {
                    'J' => (*direction == Direction::Right || *direction == Direction::Down) && side == Side::Below,
                    'L' => (*direction == Direction::Left || *direction == Direction::Down) && side == Side::Below,
                    'F' => (*direction == Direction::Left || *direction == Direction::Up) && side == Side::Above,
                    '7' => (*direction == Direction::Right || *direction == Direction::Up) && side == Side::Above,
                    _ => false,
                };
                if is_valid_traversal_out_of_main_loop {
                    stack.push(next_pos);
                    side_stack.push(Side::None);
                }
            }
        }
    }

    true
}

fn get_next_side(side: Side, current_tile: char, next_tile: char) -> Side {
    match (&side, current_tile, next_tile) {
        (Side::Above, 'F', '-') => Side::Above,
        (Side::Above, 'F', '|') => Side::Left,
        (Side::Below, 'F', '|') => Side::Right,
        (Side::Below, 'F', 'L') => Side::Above,
        (Side::Above, 'F', 'L') => Side::Below,
        (Side::Above, '7', '|') => Side::Right,
        (Side::Below, '7', '|') => Side::Left,
        (Side::Below, '7', 'J') => Side::Above,
        (Side::Above, '7', 'J') => Side::Below,
        (Side::Above, 'J', '|') => Side::Left,
        (Side::Below, 'J', '|') => Side::Right,
        (Side::Below, 'J', '7') => Side::Above,
        (Side::Above, 'J', '7') => Side::Below,
        (Side::Above, 'L', '|') => Side::Right,
        (Side::Below, 'L', '|') => Side::Left,
        (Side::Below, 'L', 'F') => Side::Above,
        (Side::Above, 'L', 'F') => Side::Below,
        (Side::Left, '|', '7') => Side::Below,
        (Side::Left, '|', 'J') => Side::Above,
        (Side::Left, '|', 'L') => Side::Below,
        (Side::Left, '|', 'F') => Side::Above,
        (Side::Right, '|', '7') => Side::Above,
        (Side::Right, '|', 'J') => Side::Below,
        (Side::Right, '|', 'L') => Side::Above,
        (Side::Right, '|', 'F') => Side::Below,
        (prev_side, _, _) => {
            *prev_side
        }
    }
}

pub fn part_two(input: &str) -> i32 {
    let pipe_grid: Vec<Vec<char>> = input.split('\n')
        .map(|line| line.chars().collect())
        .collect();

    let main_loop = find_main_loop(&pipe_grid);

    let mut inside_the_loop: HashSet<(usize, usize)> = HashSet::new();
    for (y, row) in pipe_grid.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            if is_inside_the_loop((x, y), &pipe_grid, &main_loop) {
                inside_the_loop.insert((x, y));
            }
        }
    }

    inside_the_loop.len() as i32
}


mod tests {
    use super::*;

    #[test]
    fn test_part_one_1() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....
";
        assert_eq!(part_one(input), 4);
    }

    #[test]
    fn test_part_one_2() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        assert_eq!(part_one(input), 8);
    }

    #[test]
    fn test_part_two_1() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

        assert_eq!(part_two(input), 4);
    }

    #[test]
    fn test_part_two_2() {
        let input = "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";

        assert_eq!(part_two(input), 4);
    }

    #[test]
    fn test_part_two_3() {
        let input = "\
......................
.FF7FSF7F7F7F7F7F---7.
.L|LJ||||||||||||F--J.
.FL-7LJLJ||||||LJL-77.
.F--JF--7||LJLJ7F7FJ-.
.L---JF-JLJ.||-FJLJJ7.
.|F|F-JF---7F7-L7L|7|.
.|FFJF7L7F-JF7|JL---7.
.7-L-JL7||F7|L7F-7F7|.
.L.L7LFJ|||||FJL7||LJ.
.L7JLJL-JLJLJL--JLJ.L.
......................";

        assert_eq!(part_two(input), 10);
    }

    #[test]
    fn is_inside_the_loop_1() {
        let input = "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";

        let input2 = "\
.........
..S----7.
..|F--7|.
..||..||.
..|L7FJ|.
..|.||.|.
..L-JL-J.
.........";

        let pipe_grid = input2.split('\n').map(|line| line.chars().collect()).collect::<Vec<Vec<char>>>();
        let main_loop = find_main_loop(&pipe_grid);

        // assert_eq!(false, is_inside_the_loop((4, 3), &pipe_grid, &main_loop));

        assert_eq!(true, is_inside_the_loop((3, 5), &pipe_grid, &main_loop));
    }
}