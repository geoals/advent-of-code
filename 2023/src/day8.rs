use std::collections::{HashMap, VecDeque};

use num_integer::lcm;

pub fn part_one(input: &str) -> i32 {
    let (instructions_line, node_lines) = input.split_once("\n\n").unwrap();
    let mut instructions = instructions_line.chars().collect::<VecDeque<char>>();
    let nodes = get_nodes(node_lines);

    let mut current_node = "AAA";
    let mut steps = 0;
    while !current_node.eq("ZZZ") {
        let direction = instructions.pop_front().unwrap();
        current_node = match direction {
            'L' => &nodes[current_node].left,
            'R' => &nodes[current_node].right,
            _ => panic!("Invalid instruction"),
        };
        steps += 1;
        instructions.push_back(direction);
    }
    steps
}

pub fn part_two(input: &str) -> i64 {
    let (instructions_line, node_lines) = input.split_once("\n\n").unwrap();
    let mut instructions = instructions_line.chars().collect::<VecDeque<char>>();
    let nodes = get_nodes(node_lines);

    let mut current_nodes: Vec<&str> = nodes.clone().into_keys().filter(|node| node.chars().nth(2).unwrap() == 'A').collect();
    let num_starting_nodes = current_nodes.len();
    let mut steps = 0;
    let mut completed_nodes: Vec<i64> = vec![];

    while completed_nodes.len() < num_starting_nodes {
        let direction = instructions.pop_front().unwrap();
        steps += 1;
        let mut nodes_to_remove = vec![];
        for i in 0..current_nodes.len() {
            let new_position = match direction {
                'L' => &nodes[current_nodes[i]].left,
                'R' => &nodes[current_nodes[i]].right,
                _ => panic!("Invalid instruction"),
            };
            current_nodes[i] = new_position;
            if current_nodes[i][2..3].eq("Z") {
                completed_nodes.push(steps);
                nodes_to_remove.push(i);
            }
        }
        for i in nodes_to_remove {
            current_nodes.remove(i);
        }
        instructions.push_back(direction);
    }

    find_lcm(&completed_nodes)
}

fn find_lcm(numbers: &[i64]) -> i64 {
    numbers.iter().cloned().fold(1, lcm)
}

#[derive(Debug, Clone)]
struct Node {
    left: String,
    right: String,
}

fn get_nodes(input: &str) -> HashMap<&str, Node> {
    input.lines().fold(HashMap::new(), |mut map, line| {
        map.insert(&line[0..3], Node { left: line[7..10].to_string(), right: line[12..15].to_string()});
        map
    })
}

mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
"#;

        const SAMPLE_INPUT_2: &str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
"#;

    #[test]
    fn example_input_part1() {
        assert_eq!(part_one(SAMPLE_INPUT), 2);
    }

    #[test]
    fn example_input_2_part1() {
        assert_eq!(part_one(SAMPLE_INPUT_2), 6);
    }

    const SAMPLE_INPUT_PART2: &str = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
"#;

    #[test]
    fn example_input_part2() {
        assert_eq!(part_two(SAMPLE_INPUT_PART2), 6);
    }
}