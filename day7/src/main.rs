use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum CardValue {
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

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Copy)]
struct Card {
    value: CardValue,
}

impl Card {
    fn new(type_of_card: u8) -> Self {
        match type_of_card {
            b'2' => Card {
                value: CardValue::Two,
            },
            b'3' => Card {
                value: CardValue::Three,
            },
            b'4' => Card {
                value: CardValue::Four,
            },
            b'5' => Card {
                value: CardValue::Five,
            },
            b'6' => Card {
                value: CardValue::Six,
            },
            b'7' => Card {
                value: CardValue::Seven,
            },
            b'8' => Card {
                value: CardValue::Eight,
            },
            b'9' => Card {
                value: CardValue::Nine,
            },
            b'T' => Card {
                value: CardValue::Ten,
            },
            b'J' => Card {
                value: CardValue::Jack,
            },
            b'Q' => Card {
                value: CardValue::Queen,
            },
            b'K' => Card {
                value: CardValue::King,
            },
            b'A' => Card {
                value: CardValue::Ace,
            },
            _ => panic!("WRONG CARD VALUE"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Hand {
    cards: Vec<Card>,
    htype: HandType,
    str: String,
    val: u64,
}

impl Hand {
    fn evaluate(&mut self) {
        let mut sorted_cards = self.cards.clone();
        sorted_cards.sort();

        let mut counts = HashMap::new();
        for card in &sorted_cards {
            *counts.entry(card.value).or_insert(0) += 1;
        }
        if counts.values().any(|&count| count == 5) {
            self.htype = HandType::FiveOfAKind;
            return;
        }
        if counts.values().any(|&count| count == 4) {
            self.htype = HandType::FourOfAKind;
            return;
        }
        if counts.values().any(|&count| count == 3) && counts.values().any(|&count| count == 2) {
            self.htype = HandType::FullHouse;
            return;
        }
        if counts.values().any(|&count| count == 3) {
            self.htype = HandType::ThreeOfAKind;
            return;
        }
        self.htype = match counts
            .iter()
            .filter(|&(_, &count)| count == 2)
            .collect::<Vec<_>>()
            .len()
        {
            1 => HandType::OnePair,
            2 => HandType::TwoPair,
            _ => HandType::HighCard,
        };
    }
}

impl Debug for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "str = {}, htype = {:?}", self.str, self.htype)
    }
}

fn create_hand(input: &str) -> Hand {
    let mut result = Hand {
        cards: Vec::new(),
        htype: HandType::HighCard,
        str: String::from(&input[..input.find(' ').unwrap()]),
        val: 0,
    };
    result.val = input[input.find(' ').unwrap() + 1..]
        .parse::<u64>()
        .unwrap();
    for idx in 0..input.len() {
        let c = input.as_bytes()[idx];
        if c == b' ' {
            break;
        }
        result.cards.push(Card::new(c));
    }
    result.evaluate();
    result
}

fn part1(input: &str) -> u64 {
    let mut hands: Vec<Hand> = input.lines().map(create_hand).collect();
    hands.sort_by(|a, b| match a.htype.cmp(&b.htype) {
        std::cmp::Ordering::Equal => {
            let mut idx = 0;
            while a.cards[idx].value == b.cards[idx].value {
                idx += 1;
            }
            a.cards[idx].value.cmp(&b.cards[idx].value)
        }
        _ => b.htype.cmp(&a.htype),
    });
    let mut result: u64 = hands[0].val;
    for idx in 1..hands.len() {
        let mutli = (idx + 1) as u64;
        let val = hands[idx].val;
        result += val * mutli;
    }
    result
}

fn part2(input: &str) -> u64 {
    0
}

fn main() {
    let input = include_str!("../input.txt");
    println!("part1 result = {}", part1(input));
}

#[cfg(test)]
mod test {
    use crate::part1;

    #[test]
    fn exemple_test() {
        let input = include_str!("../exemple.txt");
        assert_eq!(part1(input), 6440);
    }
}
