use std::{collections::HashSet, str::Split};

const BOARD_SIZE: usize = 5;

struct BingoResult {
    winning_score: u32,
    winning_board_index: usize,
}

fn load_bingo_setup(input: &str) -> (Split<char>, Vec<Vec<&str>>) {
    let mut input_parts = input.split("\n\n");
    let bingo_numbers = input_parts.next().unwrap().split(',');
    let boards = input_parts
        .map(|part| part.split_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    (bingo_numbers, boards)
}

fn play_bingo(bingo_numbers: Split<char>, boards: &Vec<Vec<&str>>) -> Option<BingoResult> {
    let mut marked_positions: Vec<HashSet<usize>> = vec![HashSet::new(); boards.len()];

    for bingo_number in bingo_numbers {
        for (board_num, board) in boards.iter().enumerate() {
            for (pos, num) in board.iter().enumerate() {
                if *num == bingo_number {
                    marked_positions[board_num].insert(pos);
                }
            }
        }

        for i in 0..marked_positions.len() {
            if check_win_condition(&marked_positions[i]) {
                return Some(BingoResult {
                    winning_score: sum_of_unmarked(&boards[i], &marked_positions[i])
                        * bingo_number.parse::<u32>().unwrap(),
                    winning_board_index: i,
                });
            }
        }
    }
    None
}

pub fn part_one(input: &str) -> u32 {
    let (bingo_numbers, boards) = load_bingo_setup(input);
    play_bingo(bingo_numbers, &boards).unwrap().winning_score
}

pub fn part_two(input: &str) -> u32 {
    let (bingo_numbers, mut boards) = load_bingo_setup(input);
    let number_of_boards = boards.len();

    for i in 0..boards.len() {
        let bingo_result = play_bingo(bingo_numbers.clone(), &boards).unwrap();
        boards.swap_remove(bingo_result.winning_board_index);
        if i == number_of_boards - 1 {
            return bingo_result.winning_score;
        }
    }
    0
}

fn sum_of_unmarked(board: &Vec<&str>, marked_positions: &HashSet<usize>) -> u32 {
    board
        .iter()
        .enumerate()
        .filter_map(|(i, num)| {
            if !marked_positions.contains(&i) {
                num.parse::<u32>().ok()
            } else {
                None
            }
        })
        .sum()
}

fn win_conditions() -> [HashSet<usize>; BOARD_SIZE * 2] {
    [
        HashSet::from([0, 1, 2, 3, 4]),
        HashSet::from([5, 6, 7, 8, 9]),
        HashSet::from([10, 11, 12, 13, 14]),
        HashSet::from([15, 16, 17, 18, 19]),
        HashSet::from([20, 21, 22, 23, 24]),
        HashSet::from([0, 5, 10, 15, 20]),
        HashSet::from([1, 6, 11, 16, 21]),
        HashSet::from([2, 7, 12, 17, 22]),
        HashSet::from([3, 8, 13, 18, 23]),
        HashSet::from([4, 9, 14, 19, 24]),
    ]
}

fn check_win_condition(marked_numbers: &HashSet<usize>) -> bool {
    for win_combo in win_conditions() {
        if win_combo.is_subset(&marked_numbers) {
            return true;
        }
    }
    false
}

#[test]
fn example_input_part1() {
    let input = include_str!("../day4sample.input");
    assert_eq!(part_one(input), 4512);
}

#[test]
fn example_input_part2() {
    let input = include_str!("../day4sample.input");
    assert_eq!(part_two(input), 1924);
}
