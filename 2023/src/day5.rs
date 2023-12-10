use std::ops::RangeInclusive;

struct Mapping {
    dest: i64,
    src: i64,
    length: i64,
}

impl Mapping {
    fn contains(&self, value: i64) -> bool {
        (self.src..=self.src + self.length).contains(&value)
    }

    fn get_offset(&self) -> i64 {
        self.dest - self.src
    }
}

pub fn part_one(input: &str) -> i64 {
    let mut input_chunks = input.split("\n\n");

    let seeds = input_chunks.next().unwrap()
        .split_once(": ").unwrap().1
        .split_whitespace().map(|n| n.parse().unwrap()).collect::<Vec<i64>>();

    let map_chain = [
        create_map(input_chunks.next().unwrap().split_once(":\n").unwrap().1),
        create_map(input_chunks.next().unwrap().split_once(":\n").unwrap().1),
        create_map(input_chunks.next().unwrap().split_once(":\n").unwrap().1),
        create_map(input_chunks.next().unwrap().split_once(":\n").unwrap().1),
        create_map(input_chunks.next().unwrap().split_once(":\n").unwrap().1),
        create_map(input_chunks.next().unwrap().split_once(":\n").unwrap().1),
        create_map(input_chunks.next().unwrap().split_once(":\n").unwrap().1),
    ];

    let mut locations = vec![];
    for seed in seeds {
        let mut current_value = seed;
        for map in map_chain.iter() {
            for mapping in map.iter() {
                if mapping.contains(current_value) {
                    current_value += mapping.get_offset();
                    break;
                }
            }
        }
        locations.push(current_value);
    }

    *locations.iter().min().unwrap() as i64
}

fn create_map(map_str: &str) -> Vec<Mapping> {
    map_str.lines().map(|line| {
        let mut split_line = line.split_whitespace().map(|n| n.parse::<i64>().unwrap());
        Mapping {
            dest: split_line.next().unwrap(),
            src: split_line.next().unwrap(),
            length: split_line.next().unwrap(),
        }
    }).collect()
}

pub fn part_two(input: &str) -> i64 {
    let mut input_chunks = input.split("\n\n");

    let seeds = input_chunks.next().unwrap()
        .split_once(": ").unwrap().1
        .split_whitespace().map(|n| n.parse().unwrap()).collect::<Vec<i64>>();

    let seed_ranges = seeds.chunks(2).map(|chunk| {
        chunk[0]..=chunk[0]+chunk[1]
    }).collect::<Vec<RangeInclusive<i64>>>(); 

    let map_chain = [
        create_map(input_chunks.next().unwrap().split_once(":\n").unwrap().1),
        create_map(input_chunks.next().unwrap().split_once(":\n").unwrap().1),
        create_map(input_chunks.next().unwrap().split_once(":\n").unwrap().1),
        create_map(input_chunks.next().unwrap().split_once(":\n").unwrap().1),
        create_map(input_chunks.next().unwrap().split_once(":\n").unwrap().1),
        create_map(input_chunks.next().unwrap().split_once(":\n").unwrap().1),
        create_map(input_chunks.next().unwrap().split_once(":\n").unwrap().1),
    ];

    dbg!(seed_ranges.clone());

    let mut locations = vec![];
    for seed_range in seed_ranges {
        for seed in seed_range {
            let mut current_value = seed;
            for map in map_chain.iter() {
                for mapping in map.iter() {
                    if mapping.contains(current_value) {
                        current_value += mapping.get_offset();
                        break;
                    }
                }
            }
            locations.push(current_value);
            // dbg!(locations.clone());
        }
    }

    // gaa bakelngs gjennom mappene fra location, også sjekke om det er en seed range som matcher

    *locations.iter().min().unwrap() as i64
}

mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"#;

    #[test]
    fn example_input_part1() {
        assert_eq!(part_one(SAMPLE_INPUT), 35);
    }

    #[test]
    fn example_input_part2() {
        assert_eq!(part_two(SAMPLE_INPUT), 46);
    }
}
