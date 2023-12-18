use itertools::Itertools;

// ???.### 1,1,3
pub fn part_one(input: &str) -> i64 {
    let mut sum = 0;
    let mut valid_combinations = 0;
    for (n, line) in input.lines().enumerate() {
        let (springs, remaning_hashes) = get_it(line);
        // println!("{} {}", springs, remaning_hashes);
        let combinations1 = generate_combinations(springs, remaning_hashes);
        sum += combinations1.len();
        let valid = combinations1.iter().filter(|&combination| valid_line(line, combination)).count();
        println!("line {}, valid combinations: {}", n, valid);
        valid_combinations += valid;
    }

    println!("total number of combinations {}", sum);
    valid_combinations as i64
}

fn valid_line(line: &str, combination: &[char]) -> bool {
    let rules = line.split_once(' ').unwrap().1.split(',').map(|number| number.parse::<usize>().unwrap()).collect::<Vec<usize>>();

    let mut consecutive_hashes: Vec<usize> = vec![];
    let mut count = 0;
    for c in combination {
        if *c == '#' {
            count += 1;
        } else if count != 0 {
            consecutive_hashes.push(count);
            count = 0;
        }
    }
    if count != 0 {
        consecutive_hashes.push(count);
    }

    rules == consecutive_hashes
}

fn get_it(input: &str) -> (&str, usize) {
   let (springs, numbers) = input.split_once(' ').unwrap();
    let sum_of_numbers = numbers.split(',')
        .map(|number| number.parse::<usize>().unwrap())
        .sum::<usize>();
    let number_of_damaged_springs = springs.chars().filter(|&c| c == '#').count();
    (springs, sum_of_numbers - number_of_damaged_springs)
}

// TODO only indices where there is a ?
fn generate_combinations(input: &str, replacements: usize) -> Vec<Vec<char>> {
    let mut combinations = Vec::new();
    let input_chars: Vec<char> = input.chars().collect();
    let len = input_chars.len();

    for indices in (0..len).combinations(replacements) {
        let mut current_combination = input_chars.clone();
        for &index in &indices {
            if current_combination[index] == '?' {
                current_combination[index] = '#';
            }
        }

        combinations.push(current_combination);
    }

    combinations
}

fn unfold(line: &str) -> String {
    let (springs, numbers) = line.split_once(' ').unwrap();

    let mut new_line = springs.to_string();
    for _ in 0..4 {
        new_line = format!("{}?{}", new_line, springs);
    }

    new_line = format!("{} {}", new_line, numbers);
    for _ in 0..4 {
        new_line = format!("{},{}", new_line, numbers);
    }

    new_line
}

pub fn part_two(input: &str) -> i64 {
    let unfolded_lines = input.lines().map(unfold).collect::<Vec<String>>();

    let mut sum = 0;
    let mut valid_combinations = 0;
    for (n, line) in unfolded_lines.iter().enumerate() {
        let (springs, remaning_hashes) = get_it(line);
        let combinations1 = generate_combinations(springs, remaning_hashes);
        sum += combinations1.len();
        let valid = combinations1.iter().filter(|&combination| valid_line(line, combination)).count();
        println!("line {}, valid combinations: {}", n, valid);
        valid_combinations += valid;
    }

    println!("total number of combinations {}", sum);
    valid_combinations as i64
}

mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        assert_eq!(part_one(input), 21);
    }

    #[test]
    fn test_part_one_1() {
        let input = "\
???.### 1,1,3";

        assert_eq!(part_one(input), 1);
    }

    #[test]
    fn test_part_one_2() {
        let input = ".??..??...?##. 1,1,3";

        assert_eq!(part_one(input), 4);
    }

    #[test]
    fn valid_line_1(){
        // let line = "???.### 1,1,3";
        // let combination = "#?#.###".chars().collect::<Vec<char>>();
        // assert_eq!(valid_line(line, &combination), true);

        let line2 = ".??..??...?##. 1,1,3";
        let combination2 = ".#?..#?...?###".chars().collect::<Vec<char>>();
        assert_eq!(valid_line(line2, &combination2), true);
    }

    #[test]
    fn test_unfold() {
        let input = "???.### 1,1,3";
        assert_eq!(
            "???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3",
            unfold(input)
        );
    }

    #[test]
    fn test_part_two() {

        let input = "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
       assert_eq!(525152, part_two(input));
    }

    #[test]
    fn test_part_two_1() {
        let input = ".??..??...?##. 1,1,3";
        assert_eq!(part_two(input), 16384);
    }

    #[test]
    fn test_part_two_2() {
        let input = "???.### 1,1,3";
        assert_eq!(part_two(input), 1);
    }
}
