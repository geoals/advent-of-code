use regex::Regex;

pub fn part_one(input: &str) -> i32 {
    let numbers = get_numbers(input);
    let symbols = get_symbols(input);

    numbers
        .iter()
        .filter(|number| number.adjacent_to_symbol(&symbols))
        .map(|number| number.value)
        .sum()
}

fn get_numbers(input: &str) -> Vec<Number> {
    let mut numbers: Vec<Number> = Vec::new();
    for (y, line) in input.lines().enumerate() {
        Regex::new(r"\d+").unwrap().find_iter(line).for_each(|n| {
            numbers.push(Number {
                value: n.as_str().parse::<i32>().unwrap(),
                x: (n.start() as i32 - 1, n.end() as i32),
                y: (y as i32 - 1, y as i32 + 1),
            });
        });
    }
    numbers
}

fn get_symbols(input: &str) -> Vec<Symbol> {
    let mut symbols: Vec<Symbol> = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if !c.is_digit(10) && c != '.' {
                symbols.push(Symbol {
                    x: x as i32,
                    y: y as i32,
                    value: c,
                });
            }
        }
    }
    symbols
}

#[derive(Debug, Clone)]
struct Number {
    value: i32,
    x: (i32, i32), // hitbox
    y: (i32, i32), // hitbox
}

impl Number {
    fn adjacent_to_symbol(&self, symbols: &Vec<Symbol>) -> bool {
        symbols.iter().any(|symbol| {
            (self.x.0..=self.x.1).contains(&symbol.x) && (self.y.0..=self.y.1).contains(&symbol.y)
        })
    }
}

#[derive(Debug)]
struct Symbol {
    x: i32,
    y: i32,
    value: char,
}

impl Symbol {
    fn adjacent_numbers(&self, numbers: Vec<Number>) -> Vec<Number> {
        numbers
            .into_iter()
            .filter(|number| {
                (number.x.0..=number.x.1).contains(&self.x)
                    && (number.y.0..=number.y.1).contains(&self.y)
            })
            .collect()
    }
}

pub fn part_two(input: &str) -> i32 {
    let numbers = get_numbers(input);
    let symbols = get_symbols(input);
    let gears = symbols
        .iter()
        .filter_map(|symbol| {
            let adjacent_numbers = symbol.adjacent_numbers(numbers.clone());
            if symbol.value != '*' || adjacent_numbers.len() != 2 {
                return None;
            }
            Some(adjacent_numbers)
        });

    gears
        .map(|numbers| numbers[0].value * numbers[1].value)
        .sum()
}

const SAMPLE_INPUT: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"#;

#[test]
fn example_input_part1() {
    assert_eq!(part_one(SAMPLE_INPUT), 4361);
}

#[test]
fn example_input_part2() {
    assert_eq!(part_two(SAMPLE_INPUT), 467835);
}
