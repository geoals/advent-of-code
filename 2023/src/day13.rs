pub fn part_one(input: &str) -> usize {
    let mut sum_horizontal = 0;
    let mut sum_vertical = 0;
    for pattern in input.split("\n\n") {
        let (h, v) = find_reflection(pattern);
        sum_horizontal += h;
        sum_vertical += v;
    }
    sum_vertical + (sum_horizontal * 100)
}

fn find_reflection(input: &str) -> (usize, usize) {
    let pattern: Vec<Vec<char>> = input.split('\n')
        .map(|line| line.chars().collect())
        .collect();
    let horizontal = find_horizontal_reflection(&pattern);
    if horizontal.is_some() {
        return (horizontal.unwrap(), 0);
    }
    (0, find_vertical_reflection(pattern).unwrap())
}

fn find_vertical_reflection(pattern: Vec<Vec<char>>) -> Option<usize> {
    find_horizontal_reflection(&transpose(pattern))
}

fn find_horizontal_reflection(pattern: &Vec<Vec<char>>) -> Option<usize> {
    for i in 0..pattern.len()-1 {
        for j in 0..pattern.len()-i-1 {
            if pattern[i+j+1] != pattern[i-j] {
                break;
            }
            if i - j == 0 || j == pattern.len() - i - 2 {
                return Some(i+1);
            }
        }
    }
    None
}

fn transpose(rows: Vec<Vec<char>>) -> Vec<Vec<char>> {
    (0..rows[0].len())
        .map(|col| {
            (0..rows.len())
                .map(|row| rows[row][col])
                .collect()
        }).collect()
}

pub fn part_two(input: &str) -> i32 {
    0
}

mod tests {
    use super::*;

    #[test]
    fn test_part_one_horizontal() {
        let input = "\
#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

        assert_eq!(find_reflection(input), (4, 0));
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

            assert_eq!(find_reflection(input), (0, 5));
        }

    #[test]
    fn test_part_one() {
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

        assert_eq!(part_one(input), 405);
    }
}
