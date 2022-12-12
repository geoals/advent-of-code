use std::collections::{HashSet, HashMap};

pub fn part_one(input: &str) -> usize {
    run(input, 2)
}

pub fn part_two(input: &str) -> usize {
    run(input, 10)
}

fn run(input: &str, num_knots: usize) -> usize {
    let motions = input.lines().map(|line| {
        let (direction, count ) = line.split_once(' ').unwrap();
        (direction, count.parse::<i32>().unwrap())
    });
    let mut tail_visited = HashSet::<(i32, i32)>::from([(0, 0)]);
    let mut knot_positions = vec![(0, 0); num_knots];

    for (direction, count) in motions {
        for _ in 0..count {
            for knot in 0..num_knots {
                if knot == 0 {
                    match direction {
                        "R" => knot_positions[knot].0 += 1,
                        "D" => knot_positions[knot].1 -= 1,
                        "L" => knot_positions[knot].0 -= 1,
                        "U" => knot_positions[knot].1 += 1,
                        _ => panic!()
                    }
                    continue;
                } 
                let next_knot_pos = knot_positions[knot - 1];

                if !is_adjacent(next_knot_pos, knot_positions[knot]) {
                    let direction_vector = get_direction(knot_positions[knot], next_knot_pos);
                    knot_positions[knot].0 += direction_vector.0;
                    knot_positions[knot].1 += direction_vector.1;
                    if knot == num_knots - 1 {
                        tail_visited.insert(knot_positions[knot]);
                    }
                }
            }
        }
    }

    tail_visited.len()
}

fn get_direction(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    let vector = (b.0 - a.0, b.1 - a.1);
    let x = if vector.0 == 0 { 0 } else if vector.0 > 0 { 1 } else  { -1 };
    let y = if vector.1 == 0 { 0 } else if vector.1 > 0 { 1 } else  { -1 };
    (x, y)
}

fn is_adjacent(a: (i32, i32), b: (i32, i32)) -> bool {
    ((b.0 - a.0).pow(2) as f32 + (b.1 - a.1).pow(2) as f32).sqrt() <= 2_f32.sqrt()
}

#[test]
fn example_input_part1() {
    let input = "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    assert_eq!(part_one(input), 13);
}

#[test]
fn example_input_part2() {
    let input = "\
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";
    assert_eq!(part_two(input), 36);
}

