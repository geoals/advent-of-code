use itertools::Itertools;

pub fn part_one(input: &str) -> u32 {
    let heightmap = into_double_int_vec(input);
    let low_points = find_low_points(&heightmap);
    low_points
        .iter()
        .fold(0, |acc, cur| acc + heightmap[cur.1][cur.0] + 1)
}

fn find_low_points(map: &[Vec<u32>]) -> Vec<(usize, usize)> {
    let mut low_points = vec![];
    let width = map[0].len();
    let height = map.len();

    for (x, y) in (0..height).cartesian_product(0..width) {
            let left = x == 0 || map[y][x - 1] > map[y][x];
            let right = x == width - 1 || map[y][x + 1] > map[y][x];
            let up = y == 0 || map[y - 1][x] > map[y][x];
            let down = y == height - 1 || map[y + 1][x] > map[y][x];

            if left && right && up && down {
                low_points.push((x, y));
        }
    }

    low_points
}

fn into_double_int_vec(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| l.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect()
}

pub fn part_two(input: &str) -> u32 {
    let heightmap = into_double_int_vec(input);
    let low_points = find_low_points(&heightmap);

    let mut basin_sizes = vec![];
    for basin in low_points {
        basin_sizes.push(find_size_of_basin(&heightmap, &basin));
    }
    basin_sizes.iter().sorted().rev().take(3).product()
}

fn find_size_of_basin(heightmap: &[Vec<u32>], basin: &(usize, usize)) -> u32 {
    let mut visited = vec![vec![false; heightmap[0].len()]; heightmap.len()];
    let mut size = 0;
    sum_recursive(basin.0, basin.1, &mut size, &mut visited, heightmap);
    size
}

fn sum_recursive(x: usize, y: usize, size: &mut u32, visited: &mut Vec<Vec<bool>>, heightmap: &[Vec<u32>]) {
    if visited[y][x] || heightmap[y][x] == 9 {
        return;
    }
    visited[y][x] = true;
    *size += 1;

    if x != 0 { sum_recursive(x - 1, y, size, visited, heightmap); }
    if y != 0 { sum_recursive(x, y - 1, size, visited, heightmap); }
    if x != heightmap[0].len() - 1 { sum_recursive(x + 1, y, size, visited, heightmap); }
    if y != heightmap.len() - 1 { sum_recursive(x, y + 1, size, visited, heightmap); }
}