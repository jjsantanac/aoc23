use std::{cmp::Ordering, collections::HashMap, fmt::Display};

#[derive(Debug)]
struct CamelHand {
    hand: HandType,
    bid: String,
    value: String,
}

impl Display for CamelHand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "hand: {}{}, bid: {}", self.hand, self.value, self.bid)
    }
}

impl CamelHand {
    fn type_to_digit(&self) -> i32 {
        match self.hand {
            HandType::FiveOfAKind => return 7,
            HandType::FourOfAKind => return 6,
            HandType::FullHouse => return 5,
            HandType::ThreeOfAKind => return 4,
            HandType::TwoPair => return 3,
            HandType::OnePair => return 2,
            HandType::HighCard => return 1,
        }
    }
}

#[derive(Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Display for HandType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HandType::FiveOfAKind => write!(f, "Five of a kind: ",),
            HandType::FourOfAKind => write!(f, "Four of a kind: ",),
            HandType::FullHouse => write!(f, "Full house: ",),
            HandType::ThreeOfAKind => write!(f, "Three of a kind: ",),
            HandType::TwoPair => write!(f, "Two pair: ",),
            HandType::OnePair => write!(f, "One pair: ",),
            HandType::HighCard => write!(f, "High card: ",),
        }
    }
}

pub fn solve(input: &str) {
    let mut camel_games = get_camel_games(&input, false);
    camel_games.sort_by(|a, b| camel_hand_comparator(a, b, false));

    let mut camel_games_with_jokers = get_camel_games(&input, true);
    camel_games_with_jokers.sort_by(|a, b| camel_hand_comparator(a, b, true));

    let mut result: Vec<i32> = vec![];
    let mut result_with_jokers: Vec<i32> = vec![];

    for (i, line) in camel_games.iter().enumerate() {
        result.push(line.bid.parse::<i32>().unwrap() * (i as i32 + 1))
    }

    for (i, line) in camel_games_with_jokers.iter().enumerate() {
        result_with_jokers.push(line.bid.parse::<i32>().unwrap() * (i as i32 + 1))
    }

    println!("(Part 1) RESULT: {}", result.iter().sum::<i32>());
    println!(
        "(Part 2) RESULT: {}",
        result_with_jokers.iter().sum::<i32>()
    );
}

fn get_camel_games(input: &str, with_jokers: bool) -> Vec<CamelHand> {
    return input
        .split("\n")
        .map(|l| l.split(" ").map(|s| s.trim()).collect::<Vec<&str>>())
        .filter(|l| l.len() == 2)
        .map(|l| CamelHand {
            hand: determine_hand_type(l.get(0).unwrap(), with_jokers),
            bid: l.get(1).unwrap().to_string(),
            value: l.get(0).unwrap().to_string(),
        })
        .collect();
}

fn determine_hand_type(hand: &str, with_jokers: bool) -> HandType {
    let mut occurences: HashMap<char, i32> = HashMap::new();

    for c in hand.chars() {
        match occurences.get_mut(&c) {
            Some(v) => *v += 1,
            None => {
                occurences.insert(c, 1);
            }
        }
    }

    let mut o = occurences.values().cloned().collect::<Vec<i32>>();

    if with_jokers {
        if let Some(v) = occurences.get(&'J') {
            if *v < 5 {
                o.sort();
                let i_joker = o.iter().position(|o| o == v).unwrap();
                o.remove(i_joker);

                let highest = o.last_mut().unwrap();

                *highest += v;
            }
        }
    }

    match o.len() {
        1 => return HandType::FiveOfAKind,
        2 => {
            if o.contains(&4) {
                return HandType::FourOfAKind;
            } else {
                return HandType::FullHouse;
            }
        }
        3 => {
            if o.contains(&3) {
                return HandType::ThreeOfAKind;
            } else {
                return HandType::TwoPair;
            }
        }
        4 => return HandType::OnePair,
        5 => return HandType::HighCard,
        _ => panic!("Should not happen!"),
    }
}

fn camel_hand_comparator(a: &CamelHand, b: &CamelHand, with_jokers: bool) -> Ordering {
    if a.type_to_digit() > b.type_to_digit() {
        return Ordering::Greater;
    } else if a.type_to_digit() < b.type_to_digit() {
        return Ordering::Less;
    } else {
        for (a_c, b_c) in a.value.chars().zip(b.value.chars()) {
            if a_c.eq(&b_c) {
                continue;
            } else {
                let card_labels = match with_jokers {
                    true => [
                        'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
                    ],
                    false => [
                        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
                    ],
                };

                let label_a = card_labels.iter().position(|l| l.eq(&a_c)).unwrap();
                let label_b = card_labels.iter().position(|l| l.eq(&b_c)).unwrap();

                return label_b.cmp(&label_a);
            }
        }
        return Ordering::Equal;
    }
}
