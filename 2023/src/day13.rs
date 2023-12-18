pub fn part_one(input: &str) -> usize {
    let mut sum_horizontal = 0;
    let mut sum_vertical = 0;
    for pattern in input.split("\n\n") {
        let (h, v) = find_reflection(pattern, 0);
        sum_horizontal += h;
        sum_vertical += v;
    }
    sum_vertical + (sum_horizontal * 100)
}

fn find_reflection(input: &str, ignore: usize) -> (usize, usize) {
    let pattern: Vec<Vec<char>> = input.split('\n')
        .map(|line| line.chars().collect())
        .collect();
    let horizontal = find_horizontal_reflection(&pattern, ignore);
    if horizontal.is_some() {
        return (horizontal.unwrap(), 0);
    }
    if let Some(vertical) = find_vertical_reflection(&pattern, ignore) {
        return (0, vertical)
    }
    return (0, 0);
}

fn find_vertical_reflection(pattern: &Vec<Vec<char>>, ignore: usize) -> Option<usize> {
    find_horizontal_reflection(&transpose(pattern), ignore)
}

fn find_horizontal_reflection(pattern: &Vec<Vec<char>>, ignore: usize) -> Option<usize> {
    for i in 0..pattern.len() - 1 {
        for j in 0..pattern.len() - i - 1 {
            // println!("comparing");
            // println!("{:?}", pattern[i - j]);
            // println!("{:?}", pattern[i + j + 1]);
            if pattern[i + j + 1] != pattern[i - j] || i+1 == ignore {
                break;
            }
            if i - j == 0 || j == pattern.len() - i - 2 {
                return Some(i + 1);
            }
        }
    }
    None
}

fn transpose(rows: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    (0..rows[0].len())
        .map(|col| {
            (0..rows.len())
                .map(|row| rows[row][col])
                .collect()
        }).collect()
}

pub fn part_two(input: &str) -> usize {
    let mut sum_horizontal = 0;
    let mut sum_vertical = 0;
    for pattern in input.split("\n\n") {
        println!("{}", pattern);
        let (h, v) = find_reflection(pattern, 0);
        println!("before smudge: h:{} v:{}", h, v);
        for new_pattern in smudge_positions(pattern) {

            let input = "\
#..#..#.###
###....####
##..##.#.#.
##..##.#.#.
###....####
#..#..#.###
##.##..##.#
#...#.....#
...##.###.#
####.#.##..
.#.###.#...
####..#...#
#.#.#####..
#.#.#####..
####..#...#
.#.###.#...
####.#.##..".to_string();


            if new_pattern == pattern {
                continue;
            }
            let r = new_pattern.split('\n')
                .map(|line| line.chars().collect())
                .collect();
            let result = find_vertical_reflection(&r, v);
            if result.is_some() && result.unwrap() != v {
                println!("\n{}\n", new_pattern);
                println!("found vertical reflection {}", result.unwrap());
                sum_vertical += result.unwrap();
                break;
            }
            let result = find_horizontal_reflection(&r, h);
            if input == new_pattern {
                println!("trying the correct smudge");
                println!("{:?}", result);
            }
            if result.is_some() && result.unwrap() != h {
                println!("\n{}\n", new_pattern);
                println!("found horizontal reflection {}", result.unwrap());
                sum_horizontal += result.unwrap();
                break;
            }
        }
        println!("sum h {}  sum v{} ", sum_horizontal, sum_vertical);
    }
    sum_vertical + (sum_horizontal * 100)
}

fn smudge_positions(pattern: &str) -> Vec<String> {
    // collect list of result of changing a # to . or . # for every character in pattern
    let mut result = vec![];
    for (i, c) in pattern.chars().enumerate() {
        let mut new_pattern = pattern.to_string();
        if c == '#' {
            new_pattern.replace_range(i..i + 1, ".");
        } else if c == '.' {
            new_pattern.replace_range(i..i + 1, "#");
        }
        result.push(new_pattern);
    }
    result
}

mod tests {
    use super::*;

    #[test]
    fn test_part_one_horizontal() {
        let input = "\
#..#..#.###
###....####
##..##.#.#.
##..##.#.#.
###....####
#..#..#.###
##.##..##.#
#...#.....#
...##.###.#
####.#.##..
.#.###.#...
####..#...#
#.#.#####..
#.#.#####..
####..#...#
.#.###.#...
####.#.##..";

        assert_eq!(find_reflection(input, 3), (13, 0));
    }

    #[test]
    fn test_part_one_vertical() {
        let input = "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";

        assert_eq!(find_reflection(input, 0), (0, 5));
    }

    #[test]
    fn test_part_one_and_two() {
        let input = "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

        // assert_eq!(part_one(input), 405);
        assert_eq!(part_two(input), 400);
    }

    #[test]
    fn test_part2_2() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#

.#.##.#.#
.##..##..
.#.##.#..
#......##
#......##
.#.##.#..
.##..##.#

#..#....#
###..##..
.##.#####
.##.#####
###..##..
#..#....#
#..##...#

#.##..##.
..#.##.#.
##..#...#
##...#..#
..#.##.#.
..##..##.
#.#.##.#.";
        assert_eq!(part_one(input), 709);
        assert_eq!(part_two(input), 1400);
    }
}
