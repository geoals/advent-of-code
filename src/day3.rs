use std::char;

fn get_lines(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn most_common_bit(bits: &Vec<char>) -> char {
    if bits.iter().filter(|&b| *b == '1').count() as f32 >= bits.len() as f32 / 2.0 {
        '1'
    } else {
        '0'
    }
}

fn least_common_bit(bits: &Vec<char>) -> char {
    if most_common_bit(bits) == '1' {
        '0'
    } else {
        '1'
    }
}

fn get_vertical_bit_vectors(lines: Vec<&str>) -> Vec<Vec<char>> {
    let mut bit_vecs = Vec::new();
    for (line_num, line) in lines.iter().enumerate() {
        for (bit_num, bit) in line.chars().enumerate() {
            if line_num == 0 {
                bit_vecs.push(vec![bit]);
            } else {
                bit_vecs[bit_num].push(bit);
            }
        }
    }
    bit_vecs
}

fn get_gamma_rate(input: &str) -> u32 {
    let bit_vecs = get_vertical_bit_vectors(get_lines(input));
    let mut result = String::new();
    for bits in bit_vecs {
        result.push(most_common_bit(&bits));
    }
    u32::from_str_radix(&result, 2).unwrap()
}

fn invert_bits(n: u32) -> u32 {
    let number_of_bits = (f32::log2(n as f32) + 1.0) as u32;
    !(n << (32 - number_of_bits)) >> (32 - number_of_bits)
}

fn get_epsilon_rate(gamma: u32) -> u32 {
    invert_bits(gamma)
}

fn filter_with_bit_critera(input: &str, f: &dyn Fn(&Vec<char>) -> char) -> u32 {
    let mut remaining_lines = get_lines(input);
    for i in 0..remaining_lines[0].len() {
        let remainining_bit_vecs = get_vertical_bit_vectors(remaining_lines.clone());
        let bit_critera = f(&remainining_bit_vecs[i]);
        remaining_lines = remaining_lines
            .into_iter()
            .filter(|line| line.chars().nth(i).unwrap() == bit_critera)
            .collect::<Vec<&str>>();
        if remaining_lines.len() == 1 {
            break;
        }
    }
    u32::from_str_radix(remaining_lines[0], 2).unwrap()
}

fn get_oxygen(input: &str) -> u32 {
    filter_with_bit_critera(input, &most_common_bit)
}

fn get_co2(input: &str) -> u32 {
    filter_with_bit_critera(input, &least_common_bit)
}

pub fn part_one(input: &str) -> u32 {
    let gamma = get_gamma_rate(input);
    gamma * get_epsilon_rate(gamma)
}

pub fn part_two(input: &str) -> u32 {
    get_oxygen(input) * get_co2(input)
}

#[test]
fn example_input_part1() {
    let input = include_str!("../day3sample.input");
    assert_eq!(part_one(input), 198);
}

#[test]
fn example_input_part2() {
    let input = include_str!("../day3sample.input");
    assert_eq!(part_two(input), 230);
}
