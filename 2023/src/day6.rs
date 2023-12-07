pub fn part_one(input: &str) -> i64 {
    let (times_str, distances_str) = input.split_once('\n').unwrap();
    let times: Vec<i64> = times_str.split_once("Time:").unwrap().1.split_whitespace().map(|n| n.parse().unwrap()).collect();
    let distances: Vec<i64> = distances_str.split_once("Distance:").unwrap().1.split_whitespace().map(|n| n.parse().unwrap()).collect();

    times.iter().zip(distances.iter())
        .map(number_of_ways_to_win)
        .product()
}

fn number_of_ways_to_win(time_and_distance: (&i64, &i64)) -> i64 {
    let (time, distance) = time_and_distance;
    (0..*time).map(|acceleration_time| {
        acceleration_time * (time - acceleration_time)
    })
        .filter(|&result| result > *distance)
        .count() as i64
}

pub fn part_two(input: &str) -> i64 {
    let (times_str, distances_str) = input.split_once('\n').unwrap();
    let time: i64 = times_str.split_once("Time:").unwrap().1.split_whitespace().collect::<String>().parse().unwrap();
    let distance_string = distances_str.split_once("Distance:").unwrap().1.split_whitespace().collect::<String>();
    let distance: i64 = distance_string.parse().unwrap();
    number_of_ways_to_win((&time, &distance))
}

mod tests {
    use super::*;

    const sample_input: &str = r#"Time:      7  15   30
Distance:  9  40  200
"#;

    #[test]
    fn example_input_part1() {
        assert_eq!(part_one(sample_input), 288);
    }

    #[test]
    fn example_input_part2() {
        assert_eq!(part_two(sample_input), 71503);
    }
}