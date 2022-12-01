use std::collections::HashMap;

pub fn part_one(input: &str) -> i64 {
    let paths = get_paths(input);
    for (from, to) in paths {
       println!("{} -> {:?}", from, to); 
    }
    0
}

fn search_path(paths: HashMap<&str, Vec<&str>>) {

}

fn get_paths(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut paths = HashMap::new();
    for line in input.lines() {
        let (start, end) = line.split_once('-').unwrap();
        paths.entry(start).or_insert(vec![]).push(end);
    }
    paths
}

pub fn part_two(input: &str) -> i64 {
    0
}

#[test]
fn example_part_one() {
    let input = "start-A\nstart-b\nA-c\nA-b\nb-d\nA-end\nb-end";
    assert_eq!(part_one(input), 10);
}

fn example_part_two() {
    let input = "start-A\nstart-b\nA-c\nA-b\nb-d\nA-end\nb-end";
    assert_eq!(part_two(input), 0);
}
