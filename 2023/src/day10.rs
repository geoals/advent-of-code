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
            'S' => current_position = find_next_position(&[Direction::Down, Direction::Right, Direction::Left, Direction::Up], &traversed_tiles, current_position),
            'F' => current_position = find_next_position(&[Direction::Down, Direction::Right], &traversed_tiles, current_position),
            'L' => current_position = find_next_position(&[Direction::Up, Direction::Right], &traversed_tiles, current_position),
            'J' => current_position = find_next_position(&[Direction::Up, Direction::Left], &traversed_tiles, current_position),
            '7' => current_position = find_next_position(&[Direction::Down, Direction::Left], &traversed_tiles, current_position),
            '|' => current_position = find_next_position(&[Direction::Down, Direction::Up], &traversed_tiles, current_position),
            '-' => current_position = find_next_position(&[Direction::Left, Direction::Right], &traversed_tiles, current_position),
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

fn find_next_position(
    directions: &[Direction],
    traversed_tiles: &Vec<(usize, usize)>,
    current_position: (usize, usize)
) -> (usize, usize) {
    for direction in directions {
        let new_position = move_to(direction, current_position);
        if traversed_tiles.len() > 1 && new_position == traversed_tiles[traversed_tiles.len() - 2] {
            continue;
        }
        return new_position;
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