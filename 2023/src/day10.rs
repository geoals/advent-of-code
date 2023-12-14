pub fn part_one(input: &str) -> i32 {
    let pipe_grid: Vec<Vec<char>> = input.split('\n')
        .map(|line| line.chars().collect())
        .collect();

    let starting_position = find_starting_position(&pipe_grid);

    let mut traversed_tiles: Vec<(usize, usize)> = Vec::new();

    let mut current_position = starting_position;
    loop {
        traversed_tiles.push(current_position);

        match pipe_grid[current_position.1][current_position.0] {
            'S' => current_position = get_new_position(&[Direction::Down, Direction::Right, Direction::Left, Direction::Up], &traversed_tiles, &pipe_grid, current_position),
            'F' => current_position = get_new_position(&[Direction::Down, Direction::Right], &traversed_tiles, &pipe_grid, current_position),
            'L' => current_position = get_new_position(&[Direction::Up, Direction::Right], &traversed_tiles, &pipe_grid, current_position),
            'J' => current_position = get_new_position(&[Direction::Up, Direction::Left], &traversed_tiles, &pipe_grid, current_position),
            '7' => current_position = get_new_position(&[Direction::Down, Direction::Left], &traversed_tiles, &pipe_grid, current_position),
            '|' => current_position = get_new_position(&[Direction::Down, Direction::Up], &traversed_tiles, &pipe_grid, current_position),
            '-' => current_position = get_new_position(&[Direction::Left, Direction::Right], &traversed_tiles, &pipe_grid, current_position),
            _ => {}
        }

        if traversed_tiles.len() > 1 && current_position == traversed_tiles[traversed_tiles.len() - 2] {
            println!("Can't move from {:?}:{:?}", current_position, pipe_grid[current_position.1][current_position.0]);
            break;
        }

        if current_position == starting_position {
            break;
        }
    }

    traversed_tiles.len() as i32 / 2
}

fn get_new_position(
    directions: &[Direction],
    traversed_tiles: &Vec<(usize, usize)>,
    pipe_grid: &[Vec<char>],
    current_position: (usize, usize)
) -> (usize, usize) {
    for direction in directions {
        if let Some(new_position) = move_to(&direction, pipe_grid, current_position) {
            if traversed_tiles.len() > 1 && new_position == traversed_tiles[traversed_tiles.len() - 2] {
                continue;
            }
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
    Right
}

fn move_to(direction: &Direction, pipe_grid: &[Vec<char>], current_position: (usize, usize)) -> Option<(usize, usize)> {
    let current_position_i32 = (current_position.0 as i32, current_position.1 as i32);
    let new_tile = match direction {
        Direction::Up => (current_position_i32.0, current_position_i32.1 - 1),
        Direction::Down => (current_position_i32.0, current_position_i32.1 + 1),
        Direction::Left => (current_position_i32.0 - 1, current_position_i32.1),
        Direction::Right => (current_position_i32.0 + 1, current_position_i32.1),
    };

    if new_tile.0 >= pipe_grid[0].len() as i32 || new_tile.1 >= pipe_grid.len() as i32
        || new_tile.0 < 0 || new_tile.1 < 0 {
        return None;
    }

    let new_tile = (new_tile.0 as usize, new_tile.1 as usize);

    match pipe_grid[new_tile.1][new_tile.0] {
        'S' => Some(new_tile),
        'F' if direction == &Direction::Up || direction == &Direction::Left => Some(new_tile),
        'L' if direction == &Direction::Down || direction == &Direction::Left => Some(new_tile),
        'J' if direction == &Direction::Right || direction == &Direction::Down => Some(new_tile),
        '7' if direction == &Direction::Right || direction == &Direction::Up => Some(new_tile),
        '|' if direction == &Direction::Down || direction == &Direction::Up => Some(new_tile),
        '-' if direction == &Direction::Left || direction == &Direction::Right => Some(new_tile),
        _ => None
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

pub fn part_two(input: &str) -> i32 {
    0
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
}