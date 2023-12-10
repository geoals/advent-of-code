use std::collections::HashMap;
use itertools::Itertools;

pub fn part_one(input: &str) -> i32 {
    solve(input, false)
}

pub fn part_two(input: &str) -> i32 {
    solve(input, true)
}

fn solve(input: &str, part_two: bool) -> i32 {
    get_hands(input, part_two)
        .iter()
        .sorted_by(cmp_hands)
        .enumerate().map(|(i, hand)| {
        hand.bid * (i as i32 + 1)
    }).sum()
}


#[derive(Debug, Clone)]
struct Hand {
    cards: String,
    cards_as_enum: Vec<CARD>,
    bid: i32,
    hand_type: HandType,
}

impl Hand {
    fn new(cards: String, bid: i32, part_two: bool) -> Self {
        Self {
            cards: cards.clone(),
            cards_as_enum: cards.chars().map(|c| {
                match c {
                    '2' => CARD::Two,
                    '3' => CARD::Three,
                    '4' => CARD::Four,
                    '5' => CARD::Five,
                    '6' => CARD::Six,
                    '7' => CARD::Seven,
                    '8' => CARD::Eight,
                    '9' => CARD::Nine,
                    'T' => CARD::Ten,
                    'J' => if part_two { CARD::Joker } else { CARD::Jack },
                    'Q' => CARD::Queen,
                    'K' => CARD::King,
                    'A' => CARD::Ace,
                    _ => panic!("Invalid card"),
                }
            }).collect(),
            hand_type: get_type(&cards, part_two),
            bid,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum CARD {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

fn get_type(cards: &str, part_two: bool) -> HandType {
    let num_of_each_card: HashMap<char, i32> = cards.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });
    
    if part_two && cards.contains('J') {
        return get_type_with_joker(cards);
    }

    match num_of_each_card.len() {
        5 => HandType::HighCard,
        4 => HandType::OnePair,
        3 => {
            if num_of_each_card.values().any(|&v| v == 3) {
                HandType::ThreeOfAKind
            } else {
                HandType::TwoPairs
            }
        }
        2 => {
            if num_of_each_card.values().any(|&v| v == 4) {
                HandType::FourOfAKind
            } else {
                HandType::FullHouse
            }
        }
        1 => HandType::FiveOfAKind,
        _ => panic!("Invalid hand"),
    }
}

fn get_type_with_joker(cards: &str) -> HandType {
    let mut hands: Vec<Hand> = vec![];
    for card in ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'] {
        let cards_without_joker = cards.replace('J', &card.to_string());
        hands.push(
            Hand {
                cards: cards_without_joker.clone(),
                cards_as_enum: cards_without_joker.chars().map(|c| {
                    match c {
                        '2' => CARD::Two,
                        '3' => CARD::Three,
                        '4' => CARD::Four,
                        '5' => CARD::Five,
                        '6' => CARD::Six,
                        '7' => CARD::Seven,
                        '8' => CARD::Eight,
                        '9' => CARD::Nine,
                        'T' => CARD::Ten,
                        'Q' => CARD::Queen,
                        'K' => CARD::King,
                        'A' => CARD::Ace,
                        _ => panic!("Invalid card"),
                    }
                }).collect(),
                hand_type: get_type(&cards_without_joker, true),
                bid: 0,
            }
        );
    }

    hands.iter().sorted_by(cmp_hands).last().unwrap().hand_type.clone()
}

fn get_hands(input: &str, part_two: bool) -> Vec<Hand> {
    input.lines().map(|line| {
        let (cards_str, bid) = line.split_once(' ').unwrap();
        Hand::new(cards_str.to_string(), bid.parse().unwrap(), part_two)
    }).collect()
}


fn cmp_hands(a: &&Hand, b: &&Hand) -> std::cmp::Ordering {
    if a.hand_type == b.hand_type {
        if a.cards.eq(&b.cards) {
            return std::cmp::Ordering::Equal
        }
        return cmp_hand_of_same_type(a, b);
    } else {
        a.hand_type.cmp(&b.hand_type)
    }
}   

fn cmp_hand_of_same_type(a: &&Hand, b: &&Hand) -> std::cmp::Ordering {
    for (a, b) in a.cards_as_enum.iter().zip(b.cards_as_enum.iter()) {
        if a != b {
            return a.cmp(b);
        }
    }
    std::cmp::Ordering::Equal
}

mod tests {
    use super::*;

    const sample_input: &str = r#"32T3K 765
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
            assert_eq!(part_two(sample_input), 5905);
        }
}