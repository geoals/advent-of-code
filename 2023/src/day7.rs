use std::collections::HashMap;
use crate::day7::HandType::HighCard;

struct Hand {
    cards: String,
    bid: i32,
}

enum HandType {
    HighCard = 1,
    OnePair = 2,
    TwoPairs = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

impl Hand {
    fn get_type(self) -> HandType {
        HighCard
    }
}

const cards: [char; 13] = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'K', 'Q', 'A'];
const card_to_value: HashMap<char, i32> = cards.iter().enumerate().map(|(i, &c)| (c, i as i32)).collect();

pub fn part_one(input: &str) -> i32 {

    0
}

pub fn part_two(input: &str) -> i32 {
    0
}

fn get_hands(input: &str) -> Vec<Hand> {
    input.lines().map(|line| {
        let (cards, bid) = line.split_once(' ').unwrap();
        Hand {
            cards: cards.iter().collect(),
            bid: bid.parse().unwrap(),
        }
    }).collect()
}

mod tests {
    use super::*;

    const sample_input: &str = r#"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"#;

        #[test]
        fn example_input_part1() {
            assert_eq!(part_one(sample_input), 6440);
        }

        #[test]
        fn example_input_part2() {
            assert_eq!(part_two(sample_input), 0);
        }
}