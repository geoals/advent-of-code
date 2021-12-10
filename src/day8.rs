use std::collections::HashSet;
use std::hash::Hash;

use itertools::{Itertools, MinMaxResult};

pub fn part_one(input: &str) -> usize {
    input
        .lines()
        .flat_map(|it| it.split(" | ").nth(1).unwrap().split(' '))
        .filter(|it| [2, 4, 3, 7].contains(&it.len()))
        .count()
}

pub fn part_two(input: &str) -> i64 {
    let mut sum = 0;
    for line in input.lines().collect::<Vec<&str>>() {
        sum += decode_line(line);
    }
    sum
}
fn count_in_sets<T: Eq + Hash>(sets: &[HashSet<T>], value: T) -> usize {
    sets.iter().filter(|it| it.contains(&value)).count()
}

fn decode_line(input_line: &str) -> i64 {
    let tuple = input_line.split_once(" | ").unwrap();
    let signals = tuple
        .0
        .split(' ')
        .map(|it| it.chars().sorted().collect::<String>())
        .sorted_by(|a, b| Ord::cmp(&b.len(), &a.len()));
    let output_digits: Vec<&str> = tuple.1.split(' ').collect();

    let mut possible_wire_mappings = vec![HashSet::<char>::new(); 7];

    for signal in signals.to_owned() {
        if signal.len() == 7 {
            continue;
        }
        match signal.len() {
            2 => {
                for c in signal.chars() {
                    let count = count_in_sets(&possible_wire_mappings, c);
                    if 0 < count && count < 2 {
                        continue;
                    }
                    for pos in [2, 5] {
                        let mut should_insert = true;
                        // hvis total count av den som er der fra foor er mindre enn 2, ikke insert her
                        for already_there in &possible_wire_mappings[pos] {
                            if count_in_sets(&possible_wire_mappings, *already_there) < 2 {
                                should_insert = false;
                            }
                        }
                        if should_insert {
                            if possible_wire_mappings[pos].contains(&c) {
                                // Replace set at position pos with the chars of current signal
                                let mut this_signal_as_set = HashSet::new();
                                for c in signal.chars() {
                                    this_signal_as_set.insert(c);
                                }
                                let current_set_copy = possible_wire_mappings[pos].to_owned();
                                let intersection = current_set_copy
                                    .intersection(&this_signal_as_set)
                                    .to_owned();
                                possible_wire_mappings[pos].clear();
                                for val in intersection {
                                    possible_wire_mappings[pos].insert(*val);
                                }
                            } else {
                                possible_wire_mappings[pos].insert(c);
                            }
                        }
                    }
                    for pos in [0, 1, 3, 4, 6] {
                        possible_wire_mappings[pos].remove(&c);
                    }
                }
            }
            3 => {
                for c in signal.chars() {
                    let count = count_in_sets(&possible_wire_mappings, c);
                    if 0 < count && count < 3 {
                        continue;
                    }
                    for pos in [0, 2, 5] {
                        let mut should_insert = true;
                        // hvis total count av den som er der fra foor er mindre enn 3, ikke insert her
                        for already_there in &possible_wire_mappings[pos] {
                            if count_in_sets(&possible_wire_mappings, *already_there) < 3 {
                                should_insert = false;
                            }
                        }
                        if should_insert {
                            if possible_wire_mappings[pos].contains(&c) {
                                // Replace set at position pos with the chars of current signal
                                let mut this_signal_as_set = HashSet::new();
                                for c in signal.chars() {
                                    this_signal_as_set.insert(c);
                                }
                                let current_set_copy = possible_wire_mappings[pos].to_owned();
                                let intersection = current_set_copy
                                    .intersection(&this_signal_as_set)
                                    .to_owned();
                                possible_wire_mappings[pos].clear();
                                for val in intersection {
                                    possible_wire_mappings[pos].insert(*val);
                                }
                            } else {
                                possible_wire_mappings[pos].insert(c);
                            }
                        }
                    }
                    for pos in [1, 3, 4, 6] {
                        possible_wire_mappings[pos].remove(&c);
                    }
                }
            }
            4 => {
                for c in signal.chars() {
                    let count = count_in_sets(&possible_wire_mappings, c);
                    if 0 < count && count < 4 {
                        continue;
                    }
                    for pos in [1, 2, 3, 5] {
                        let mut should_insert = true;
                        // hvis total count av den som er der fra foor er mindre enn 4, ikke insert her
                        for already_there in &possible_wire_mappings[pos] {
                            if count_in_sets(&possible_wire_mappings, *already_there) < 4 {
                                should_insert = false;
                            }
                        }
                        if should_insert {
                            if possible_wire_mappings[pos].contains(&c) {
                                // Replace set at position pos with the chars of current signal
                                let mut this_signal_as_set = HashSet::new();
                                for c in signal.chars() {
                                    this_signal_as_set.insert(c);
                                }
                                let current_set_copy = possible_wire_mappings[pos].to_owned();
                                let intersection = current_set_copy
                                    .intersection(&this_signal_as_set)
                                    .to_owned();
                                possible_wire_mappings[pos].clear();
                                for val in intersection {
                                    possible_wire_mappings[pos].insert(*val);
                                }
                            } else {
                                possible_wire_mappings[pos].insert(c);
                            }
                        }
                    }
                    for pos in [0, 4, 6] {
                        possible_wire_mappings[pos].remove(&c);
                    }
                }
            }
            5 | 6 | 7 => {
                for c in signal.chars() {
                    let count = count_in_sets(&possible_wire_mappings, c);
                    if count > 0 {
                        continue;
                    }
                    for pos in &mut possible_wire_mappings {
                        pos.insert(c);
                    }
                }
            }
            _ => (),
        }
    }

    for signal in signals {
        // use all chars in this signal to check out possible numbers according to current possibilities
        // ex. current possibilities: [d, ef, ab, ef, cg, ab, cg], a signal of abcdef MUST be a 9, so now we know where c and g is
        let mut possible_formations = HashSet::new();
        for permutation in (0..7).permutations(7) {
            let mut formation = vec![];
            for c in signal.chars() {
                for i in &permutation {
                    if possible_wire_mappings[*i].contains(&c) && !formation.contains(i) {
                        formation.push(*i);
                        break;
                    }
                }
            }

            formation.sort();
            possible_formations.insert(formation);
        }

        println!("{:?}", &possible_formations);

        // trenger bare teste 2,3,5 hvis lengden er 5 osv.
        for formation in possible_formations {
            println!("segments for {}: {:?}", signal, formation);
            if my_eq(&formation, &[0, 2, 5]) {
                // 7
                println!("we have a 7");
                for pos_not_part_of_formation in [1, 3, 4, 6] {
                    for x in possible_wire_mappings[pos_not_part_of_formation].to_owned() {
                        if !signal.contains(x) {
                            for set_index in [0, 2, 5] {
                                possible_wire_mappings[set_index].remove(&x);
                                println!(
                                    "Removed {} from {} because its not part of this formation",
                                    &x, set_index
                                );
                            }
                        }
                    }
                }
            } else if my_eq(&formation, &[2, 5]) {
                // 1
                println!("we have a 1");
                for pos_not_part_of_formation in [0, 1, 3, 4, 6] {
                    for x in possible_wire_mappings[pos_not_part_of_formation].to_owned() {
                        if !signal.contains(x) {
                            for set_index in [2, 5] {
                                possible_wire_mappings[set_index].remove(&x);
                                println!(
                                    "Removed {} from {} because its not part of this formation",
                                    &x, set_index
                                );
                            }
                        }
                    }
                }
            } else if my_eq(&formation, &[0, 2, 3, 4, 6]) {
                // 2
                println!("we have a 2");
                for pos_not_part_of_formation in [1, 5] {
                    for x in possible_wire_mappings[pos_not_part_of_formation].to_owned() {
                        if !signal.contains(x) {
                            for set_index in [0, 2, 3, 4, 6] {
                                possible_wire_mappings[set_index].remove(&x);
                                println!(
                                    "Removed {} from {} because its not part of this formation",
                                    &x, set_index
                                );
                            }
                        }
                    }
                }
            } else if my_eq(&formation, &[0, 2, 3, 5, 6]) {
                //3
                println!("we have a 3");
                for pos_not_part_of_formation in [1, 4] {
                    for x in possible_wire_mappings[pos_not_part_of_formation].to_owned() {
                        if !signal.contains(x) {
                            for set_index in [0, 2, 3, 5, 6] {
                                possible_wire_mappings[set_index].remove(&x);
                                println!(
                                    "Removed {} from {} because its not part of this formation",
                                    &x, set_index
                                );
                            }
                        }
                    }
                }
            } else if my_eq(&formation, &[1, 2, 3, 5]) {
                //4
                println!("we have a 4");
                for pos_not_part_of_formation in [0, 1, 4, 6] {
                    for x in possible_wire_mappings[pos_not_part_of_formation].to_owned() {
                        if !signal.contains(x) {
                            for set_index in [1, 2, 3, 5] {
                                possible_wire_mappings[set_index].remove(&x);
                                println!(
                                    "Removed {} from {} because its not part of this formation",
                                    &x, set_index
                                );
                            }
                        }
                    }
                }
            } else if my_eq(&formation, &[0, 1, 3, 5, 6]) {
                //5
                println!("we have a 5");
                for pos_not_part_of_formation in [2, 4] {
                    for x in possible_wire_mappings[pos_not_part_of_formation].to_owned() {
                        if !signal.contains(x) {
                            for set_index in [0, 1, 3, 5, 6] {
                                possible_wire_mappings[set_index].remove(&x);
                                println!(
                                    "Removed {} from {} because its not part of this formation",
                                    &x, set_index
                                );
                            }
                        }
                    }
                }
            } else if my_eq(&formation, &[0, 1, 3, 4, 5, 6]) {
                //6
                println!("we have a 6");
                for pos_not_part_of_formation in [2] {
                    for x in possible_wire_mappings[pos_not_part_of_formation].to_owned() {
                        if !signal.contains(x) {
                            for set_index in [0, 1, 3, 4, 5, 6] {
                                possible_wire_mappings[set_index].remove(&x);
                                println!(
                                    "Removed {} from {} because its not part of this formation",
                                    &x, set_index
                                );
                            }
                        }
                    }
                }
            } else if my_eq(&formation, &[0, 1, 2, 3, 4, 5, 6]) {
                // 8
                println!("we have a 8");
            } else if my_eq(&formation, &[0, 1, 2, 3, 5, 6]) {
                // 9
                println!("we have a 9");
                // letters in position 4 (not part of 9) which are not in signal, should be removed from positions which consitute the 9 formation
                // we have cg in position 4, so we remove g from position 6
                // if we now only have one letter in position 6 (yes, c), we can remove this letter for position 4
                for pos_not_part_of_formation in [4] {
                    for x in possible_wire_mappings[pos_not_part_of_formation].to_owned() {
                        if !signal.contains(x) {
                            for set_index in [0, 1, 2, 3, 5, 6] {
                                possible_wire_mappings[set_index].remove(&x);
                                println!(
                                    "Removed {} from {} because its not part of this formation",
                                    &x, set_index
                                );
                            }
                        }
                    }
                }
            } else if my_eq(&formation, &[0, 1, 2, 4, 5, 6]) {
                // 0
                println!("we have a 0");
                for pos_not_part_of_formation in [3] {
                    for x in possible_wire_mappings[pos_not_part_of_formation].to_owned() {
                        if !signal.contains(x) {
                            for set_index in [0, 1, 2, 4, 5, 6] {
                                possible_wire_mappings[set_index].remove(&x);
                                println!(
                                    "Removed {} from {} because its not part of this formation",
                                    &x, set_index
                                );
                            }
                        }
                    }
                }
            }
        }

        println!("{:?}", possible_wire_mappings);
    }

    let sets_copy = possible_wire_mappings.to_owned();
    println!("remove the last doubles from {:?}", sets_copy);
    // remove the last double
    for set in &mut possible_wire_mappings {
        if set.len() > 1 {
            for c in set.to_owned() {
                for setj in &sets_copy {
                    if setj.len() == 1 && setj.contains(&c) {
                        set.remove(&c);
                        break;
                    }
                }
            }
        }
    }

    println!("{:#?}", possible_wire_mappings);
    let mut segment_positions = ['0'; 7];
    for (i, set) in possible_wire_mappings.iter().enumerate() {
        segment_positions[i] = *set.iter().next().unwrap();
    }
    println!("Final segment positions: {:?}", segment_positions);

    let mut output_value = vec![];
    for out_digit in output_digits {
        let sorted = &out_digit.chars().sorted().collect::<String>()[..];
        println!("Sorted output chars: {}", sorted);
        let mut indices = vec![];
        for c in sorted.chars() {
            indices.push(segment_positions.iter().position(|&x| x == c).unwrap());
        }
        indices.sort();
        println!("Indices into segment table: {:?}", indices);

        println!("input line {}", input_line);

        let num = match &indices[..] {
            [0, 1, 2, 4, 5, 6] => 0,
            [2, 5] => 1,
            [0, 2, 3, 4, 6] => 2,
            [0, 2, 3, 5, 6] => 3,
            [1, 2, 3, 5] => 4,
            [0, 1, 3, 5, 6] => 5,
            [0, 1, 3, 4, 5, 6] => 6,
            [0, 2, 5] => 7,
            [0, 1, 2, 3, 4, 5, 6] => 8,
            [0, 1, 2, 3, 5, 6] => 9,
            num => {
                panic!("{:?}", num);
            }
        };

        output_value.push(num);
    }

    println!("{:?}", output_value);
    let output_value = output_value
        .iter()
        .map(|&n| n.to_string())
        .collect::<String>()
        .parse::<i32>()
        .unwrap();

    output_value as i64
}

fn my_eq<T>(a: &[T], b: &[T]) -> bool
where
    T: Eq + Hash,
{
    let a: HashSet<_> = a.iter().collect();
    let b: HashSet<_> = b.iter().collect();

    a == b
}

fn decode_line_attempt_2(input_line: &str) -> i64 {
    let tuple = input_line.split_once(" | ").unwrap();
    let signals = tuple
        .0
        .split(' ')
        .map(|it| it.chars().sorted().collect::<String>())
        .sorted_by(|a, b| Ord::cmp(&a.len(), &b.len()));
    let output_digits: Vec<&str> = tuple.1.split(' ').collect();

    let mut possible_wire_mappings = vec![HashSet::<char>::new(); 7];

    for signal in signals {
        let numbers = find_number(&signal, possible_wire_mappings.to_owned());
        println!("{} matched numbers {:?}", signal, numbers);
        for number in numbers {
            mark_number_in_wire_mappings(&signal, number, &mut possible_wire_mappings);
        }
        println!("Possible wire mappings: {:?}", possible_wire_mappings);
        // let mut permutation_base = [0; 7];
        // for i in 0..signal.len() {
        //     permutation_base[i] = 1
        // }
        // let mut matching_numbers = HashSet::new();
        // // TODO dont need to brute force this as the valid combinations are known statically
        // for permutation in permutation_base.iter().permutations(7) {
        //     let number = match permutation[..] {
        //         [1, 1, 1, 0, 1, 1, 1] => 0,
        //         [0, 0, 1, 0, 0, 1, 0] => 1,
        //         [1, 0, 1, 1, 1, 0, 1] => 2,
        //         [1, 0, 1, 1, 0, 1, 1] => 3,
        //         [0, 1, 1, 1, 0, 1, 0] => 4,
        //         [1, 1, 0, 1, 0, 1, 1] => 5,
        //         [1, 1, 0, 1, 1, 1, 1] => 6,
        //         [1, 0, 1, 0, 0, 1, 0] => 7,
        //         [1, 1, 1, 1, 1, 1, 1] => 8,
        //         [1, 1, 1, 1, 0, 1, 1] => 9,
        //         _ => -1,
        //     };
        //     if number != -1 {
        //         matching_numbers.insert((number, permutation));
        //     }
        // }
        // let letter_permu: Vec<Vec<char>> = signal.chars().permutations(signal.len()).collect();
        // // we want permutations : [0, 0, b, 0, 0, e, 0] and [0, 0, e, 0, 0, b, 0]

        // println!("letter permutations : {:?}", letter_permu);
        // println!("Matching numbers: {:?}", matching_numbers);
    }

    0
}

fn mark_number_in_wire_mappings(
    signal: &str,
    number: i64,
    possible_wire_mappings: &mut [HashSet<char>],
) {
    let segment_positions = match number {
        0 => [1, 1, 1, 0, 1, 1, 1],
        1 => [0, 0, 1, 0, 0, 1, 0],
        2 => [1, 0, 1, 1, 1, 0, 1],
        3 => [1, 0, 1, 1, 0, 1, 1],
        4 => [0, 1, 1, 1, 0, 1, 0],
        5 => [1, 1, 0, 1, 0, 1, 1],
        6 => [1, 1, 0, 1, 1, 1, 1],
        7 => [1, 0, 1, 0, 0, 1, 0],
        8 => [1, 1, 1, 1, 1, 1, 1],
        9 => [1, 1, 1, 1, 0, 1, 1],
        _ => panic!("illegal number"),
    };

    let previous_wire_mappings = possible_wire_mappings.to_owned();
    let mut chars_to_insert = vec![];
    for c in signal.chars() {
        let count = count_in_sets(&previous_wire_mappings, c);
        if count == 0 || count > signal.len() {
            chars_to_insert.push(c);
        }
    }
    if chars_to_insert.is_empty() {
        return;
    }
    println!("chars to insert: {:?}", &chars_to_insert);

    let mut position_to_insert_into = HashSet::new();
    for (i, segment_position) in segment_positions.iter().enumerate() {
        if *segment_position == 0 {
            continue;
        }
        for c in &chars_to_insert {
            let count = previous_wire_mappings[i].len();
            if count == 0 || count > signal.len() {
                position_to_insert_into.insert(i);
            }
        }
    }
    println!("positions to insert into: {:?}", &position_to_insert_into);

    for i in position_to_insert_into {
        possible_wire_mappings[i] = HashSet::from_iter(chars_to_insert.to_owned());
    }

    // segment positions: ex 2: [1,0,1,1,1,0,1]
    // clean up according to number
    // wire mappings: [{'d'}, {'g', 'c'}, {'e', 'b'}, {'g', 'c'}, {'a', 'f'}, {'e', 'b'}, {'a', 'f'}]
    // signal: abcdf
    // we should end up with [{'d'}, {g c}, {'b'}, {'c'}, {'a', 'f'}, {e b}, {'a', 'f'}]
    // do a cleanup sweep
    println!("segment positions {:?}", segment_positions);
    println!("wire mappings {:?}", possible_wire_mappings);
    println!("signal {:?}", signal);
    let mut minimal_number_formation = vec![];
    for (i, seg_pos) in segment_positions.into_iter().enumerate() {
        match seg_pos {
            0 => minimal_number_formation.push(possible_wire_mappings[i].to_owned()),
            1 => {
                // but first remove chars not in signal
                let signal_as_set: HashSet<char> = HashSet::from_iter(signal.chars());
                let a = possible_wire_mappings[i].intersection(&signal_as_set);
                // println!("intersection of {:?} and {:?} {:?} {:?}", signal_as_set, possible_wire_mappings[seg_pos], seg_pos, possible_wire_mappings);
                let mut new_one: HashSet<char> = HashSet::new();
                for c in a {
                    new_one.insert(*c);
                }
                minimal_number_formation.push(new_one);
            }
            _ => (),
        }
    }
    
    
    let sets_copy = minimal_number_formation.to_owned();
    println!("remove the last doubles from {:?}", sets_copy);
    // remove the last double
    for set in &mut minimal_number_formation {
        if set.len() > 1 {
            for c in set.to_owned() {
                for setj in &sets_copy {
                    if setj.len() == 1 && setj.contains(&c) {
                        set.remove(&c);
                        break;
                    }
                }
            }
        }
    }

    println!("minimal number formation: {:?}", minimal_number_formation);
    // replace possible wire mappings with minimal numbe fomration
    let mut i = 0;
    for set in minimal_number_formation {
        possible_wire_mappings[i].clear();
        for c in set {
            possible_wire_mappings[i].insert(c);
        }
        i += 1;
    }
}

fn find_number(signal: &str, possible_wire_mappings: Vec<HashSet<char>>) -> Vec<i64> {
    match signal.len() {
        2 => return vec![1],
        3 => return vec![7],
        4 => return vec![4],
        5 => return vec![2,3,5],
        6 => return vec![0,6,9],
        7 => return vec![8],
        _ => (),
    }
    todo!()
}

#[test]
fn example_part1() {
    assert_eq!(part_one(include_str!("day8sample.input")), 26)
}

#[test]
fn example_part2() {
    // decode_line("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf");
    // decode_line("efbag de adcgf fgcdea gfdabc daec fdcgbe edg fgead acbgdfe | de bgfae decafbg gefabcd");
    // decode_line_attempt_2("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe");
    // decode_line_attempt_2("be edb cgeb fabcd fecdb fdcge agebfd | fdgacbe cefdb cefbgd gcbe");
    // decode_line_attempt_2("be edb cgeb fabcd | fdgacbe cefdb cefbgd gcbe");
    // assert_eq!(
    //     decode_line("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe"),
    //     8394
    // );
    decode_line_attempt_2("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf");
    // assert_eq!(part_two(include_str!("day8sample.input")), 61229)
}
