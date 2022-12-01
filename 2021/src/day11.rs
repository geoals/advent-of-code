use std::thread;

use itertools::Itertools;

pub fn part_one(input: &str) -> i64 {
    let mut grid = into_double_int_vec(input);
    let mut flashes = 0;

    for _ in 0..100 {
        flashes += step(&mut grid);
        // print_grid(&grid);
        // print!("{}[2J", 27 as char);
        // thread::sleep_ms(50);
    }
    print_grid(&grid);

    flashes
}

fn step(grid: &mut Vec<Vec<u32>>) -> i64 {
    let width = grid[0].len();
    let height = grid.len();
    let mut num_flashes = 0;
    let mut flashed = vec![vec![false; grid[0].len()]; grid.len()];
    for (x, y) in (0..height).cartesian_product(0..width) {
        grid[y][x] += 1;
    }

    loop {
        let mut remaining_flashers = 0;
        for (x, y) in (0..height).cartesian_product(0..width) {
            if grid[y][x] > 9 && !flashed[y][x] {
                flashed[y][x] = true;
                num_flashes += 1;
                if x != 0 { grid[y][x-1] += 1 };
                if y != 0 { grid[y-1][x] += 1 };
                if x < width - 1 { grid[y][x+1] += 1 };
                if y < height - 1 { grid[y+1][x] += 1 };
                if x != 0 && y != 0 { grid[y-1][x-1] += 1 };
                if x != 0 && y < height - 1 { grid[y+1][x-1] += 1 };
                if x < width - 1 && y != 0 { grid[y-1][x+1] += 1 };
                if x < width - 1&& y < width - 1{ grid[y+1][x+1] += 1 };
            }
        }

        for (x, y) in (0..height).cartesian_product(0..width) {
            if grid[y][x] > 9 && !flashed[y][x] {
                remaining_flashers += 1;
            }
        }
        if remaining_flashers == 0 {
            break;
        }
    }

    for (x, y) in (0..height).cartesian_product(0..width) {
        if flashed[y][x] { grid[y][x] = 0 }
    }
    num_flashes
}

pub fn part_two(input: &str) -> i64 {
    let mut grid = into_double_int_vec(input);

    let mut i = 1;
    loop {
        let flashes = step(&mut grid);
        if flashes == (grid[0].len() * grid.len()) as i64 {
            break;
        }
        i += 1;
    }
    print_grid(&grid);
    i
}

fn into_double_int_vec(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| l.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect()
}

fn print_grid(grid: &[Vec<u32>]) {
    for vertical in grid {
        println!("{:?}", vertical);
    }
    println!("");
}

#[test]
fn example_part_one() {
    let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
    assert_eq!(part_one(input), 1656);
}

#[test]
fn example_part_two() {
    let input = "";
    assert_eq!(part_two(input), 0);
}