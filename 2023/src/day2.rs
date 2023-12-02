const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE: i32 = 14;

pub fn part_one(input: &str) -> i32 {
    input.lines()
        .map(|line| line.split_once(": ").unwrap())
        .filter(valid_game)
        .map(to_game_id)
        .sum()
}

fn valid_game((_, game): &(&str, &str)) -> bool {
    game.split("; ").all(valid_draw)
}

fn valid_draw(draw: &str) -> bool {
    draw.split(", ").all(valid_result)
}

fn valid_result(result: &str) -> bool {
    let (num, color) = result.split_once(' ').unwrap();
    let num_i32 = num.parse::<i32>().unwrap();
    match color {
        "red" if num_i32 > MAX_RED => return false,
        "green" if num_i32 > MAX_GREEN => return false,
        "blue" if num_i32 > MAX_BLUE => return false,
        _ => true,
    };
    true
}

fn to_game_id((game_num, _): (&str, &str)) -> i32 {
    game_num.split_once(' ').unwrap().1.parse::<i32>().unwrap()
}


pub fn part_two(input: &str) -> i32 {
    input.lines()
        .map(power)
        .sum()
}

fn power(game: &str) -> i32 {
    let game = game.split_once(": ").unwrap().1;
    let draws = game.split(|c| c == ',' || c == ';').collect();

    let blue = min_amount_required_of_color("blue", &draws);
    let green = min_amount_required_of_color("green", &draws);
    let red = min_amount_required_of_color("red", &draws);

    blue * green * red
}

fn min_amount_required_of_color(color: &str, draws: &Vec<&str>) -> i32 {
    draws
        .iter()
        .filter(|draw| draw.ends_with(color))
        .map(|draw| draw.trim().split_once(' ').unwrap().0.parse::<i32>().unwrap())
        .max().unwrap()
}

const sample_input: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"#;

#[test]
fn example_input_part1() {
    assert_eq!(part_one(sample_input), 8);
}

#[test]
fn example_input_part2() {
    assert_eq!(part_two(sample_input), 2286);
}