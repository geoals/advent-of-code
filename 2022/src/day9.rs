use std::collections::{HashSet, HashMap};

pub fn part_one(input: &str) -> usize {
    let motions = input.lines().map(|line| {
        let (direction, count ) = line.split_once(' ').unwrap();
        (direction, count.parse::<i32>().unwrap())
    });

    let mut previous_head_pos;
    let mut head_pos = (0, 0);
    let mut tail_pos = (0, 0);
    let mut tail_visited = HashSet::<(i32, i32)>::from([tail_pos]);

    for (direction, count) in motions {
        for _ in 0..count {
            previous_head_pos = head_pos;
            match direction {
                "R" => head_pos.0 += 1,
                "D" => head_pos.1 -= 1,
                "L" => head_pos.0 -= 1,
                "U" => head_pos.1 += 1,
                _ => panic!()
            }
            if !is_adjacent(head_pos, tail_pos) { 
                tail_pos = previous_head_pos; 
                tail_visited.insert(tail_pos);
            }
        }
    }

    tail_visited.len()
}

fn is_adjacent(a: (i32, i32), b: (i32, i32)) -> bool {
    ((b.0 - a.0).pow(2) as f32 + (b.1 - a.1).pow(2) as f32).sqrt() <= 2_f32.sqrt()
}

pub fn part_two(input: &str) -> usize {
    let motions = input.lines().map(|line| {
        let (direction, count ) = line.split_once(' ').unwrap();
        (direction, count.parse::<i32>().unwrap())
    });

    let mut tail_visited = HashSet::<(i32, i32)>::from([(0, 0)]);
    let mut knot_positions: HashMap<i32, (i32, i32)>= (0..10).map(|knot| {
        (knot, (0, 0))
    }).collect();


    for (direction, count) in motions {

        for _ in 0..count {
            let mut previous_head_pos = (0, 0);

            for knot in 0..10 {
                let inc = if knot != 0 { 1 } else { 0 };
                let mut head_pos = knot_positions[&(knot - inc)];
                let mut tail_pos = knot_positions[&knot];

                // special handling for head: 
                if knot == 0 {
                    let prev = head_pos;

                    match direction {
                        "R" => head_pos.0 += 1,
                        "D" => head_pos.1 -= 1,
                        "L" => head_pos.0 -= 1,
                        "U" => head_pos.1 += 1,
                        _ => panic!()
                    }
                    knot_positions.insert(knot, head_pos);
                    println!("Moved knot {:?} from {:?} to {:?} ", knot, prev, head_pos);
                } else {
                    if is_adjacent(head_pos, tail_pos) {
                        continue
                    }
                    let prev = tail_pos;
                    if !is_adjacent(head_pos, tail_pos) && (head_pos.0 == tail_pos.0 || head_pos.1 == tail_pos.1) { // same row or col
                        match direction {
                            "R" => tail_pos.0 += 1,
                            "D" => tail_pos.1 -= 1,
                            "L" => tail_pos.0 -= 1,
                            "U" => tail_pos.1 += 1,
                            _ => panic!()
                        }
                        knot_positions.insert(knot, tail_pos);
                        println!("Moved knot {:?} from {:?} to {:?} ", knot, prev, tail_pos);
                    } else {
                        // move diagonal in direction of knotnr knot-1
                        println!("knot {} at {:?} should go diagonal now towards {:?}", knot, tail_pos, head_pos);
                    }
                    // let temp = previous_head_pos;
                    // previous_head_pos = tail_pos;
                    // println!("head {:?} tail {:?} knot {}", head_pos, tail_pos, knot);
                    // println!("Finding next position for knot[{}], currently at {:?}", knot, tail_pos);
                    // tail_pos = temp;
                    // knot_positions.insert(knot, tail_pos);
                    // println!("Moved knot {:?} from {:?} to {:?} ", knot, previous_head_pos, temp);
                    // if !is_adjacent(head_pos, tail_pos) { 
                    //     // tail_pos = previous_head_pos; 
                    //     tail_visited.insert(tail_pos);
                    // }
                    // println!("{:?}", knot_positions);
                }
            }
        }
    }

    tail_visited.len()
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
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    assert_eq!(part_two(input), 13);
}

