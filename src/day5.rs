pub fn part_one(input: &str) -> usize {
    let lines = read_into_lines(input);
    let mut coord_system = vec![vec![0; 1000]; 1000];
    for line in lines {
        increment_on_line_segment(&line, &mut coord_system)
    }
    coord_system.iter().flatten().filter(|&x| x > &1).count()
}

fn increment_on_line_segment(line: &Line, coord_system: &mut Vec<Vec<usize>>) {
    if line.orientation == Orientation::Horizontal {
        for x in line.start.0..=line.end.0 {
            let y = line.start.1;
            coord_system[y][x] += 1;
        }
    } else if line.orientation == Orientation::Vertical {
        for y in line.start.1..=line.end.1 {
            let x = line.start.0;
            coord_system[y][x] += 1;
        }
    }
}

pub fn part_two(input: &str) -> usize {
    let lines = read_into_lines(input);
    let mut coord_system = vec![vec![0; 1000]; 1000];

    for line in lines {
        increment_on_line_segment(&line, &mut coord_system);

        if line.orientation == Orientation::Diagonal {
            // Diagonal from top left to bottom right or opposite
            for y in line.start.1..=line.end.1 {
                for x in line.start.0..=line.end.0 {
                    if x - line.start.0 == y - line.start.1 {
                        coord_system[y][x] += 1;
                    }
                }
            }
            // Diagonal from bottom left to top right
            for (j, y) in (line.end.1..=line.start.1).enumerate() {
                for x in line.start.0..=line.end.0 {
                    if (x as i32) - (line.end.0 as i32) + (j as i32)
                        == (y as i32) - (line.end.1 as i32) - (j as i32)
                    {
                        coord_system[y][x] += 1;
                    }
                }
            }
            // Diagonal from top right to bottom left
            for (j, y) in (line.start.1..=line.end.1).enumerate() {
                for x in line.end.0..=line.start.0 {
                    if (x as i32) - (line.start.0 as i32) + (j as i32)
                        == (y as i32) - (line.start.1 as i32) - (j as i32)
                    {
                        coord_system[y][x] += 1;
                    }
                }
            }
        }
    }
    coord_system.iter().flatten().filter(|&x| x > &1).count()
}

#[derive(Debug)]
struct Line {
    orientation: Orientation,
    start: (usize, usize),
    end: (usize, usize),
}

#[derive(Debug, PartialEq)]
enum Orientation {
    Horizontal,
    Vertical,
    Diagonal,
}

// This needs refactoring :/
fn read_into_lines(input: &str) -> Vec<Line> {
    let mut lines = vec![];
    let input_lines = input
        .split('\n')
        .map(|line| line.split(" -> ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    for input_line in input_lines {
        let coords = input_line
            .iter()
            .map(|xy| xy.split(',').collect::<Vec<&str>>())
            .flatten()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let (start, end) = match coords[0] < coords[2] || coords[1] < coords[3] {
            true => ((coords[0], coords[1]), (coords[2], coords[3])),
            false => ((coords[2], coords[3]), (coords[0], coords[1])),
        };
        let orientation = if start.0 == end.0 {
            Orientation::Vertical
        } else if start.1 == end.1 {
            Orientation::Horizontal
        } else {
            Orientation::Diagonal
        };

        lines.push(Line {
            start,
            end,
            orientation,
        });
    }
    lines
}

#[test]
fn example_input_part1() {
    let input = include_str!("../day5sample.input");
    assert_eq!(part_one(input), 5);
}

#[test]
fn example_input_part2() {
    let input = include_str!("../day5sample.input");
    assert_eq!(part_two(input), 12);
}

#[test]
fn example_input_part2_test1() {
    let input = include_str!("../day5test1.input");
    assert_eq!(part_two(input), 6);
}
