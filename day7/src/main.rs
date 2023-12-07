use std::collections::HashMap;

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

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
struct Card {
    value: CardValue,
}

impl Card {
    fn new(type_of_card: u8) -> Self {
        match type_of_card {
            b'2' => Card { value: CardValue::Two },
            b'3' => Card { value: CardValue::Three },
            b'4' => Card { value: CardValue::Four },
            b'5' => Card { value: CardValue::Five },
            b'6' => Card { value: CardValue::Six },
            b'7' => Card { value: CardValue::Seven },
            b'8' => Card { value: CardValue::Eight },
            b'9' => Card { value: CardValue::Nine },
            b'T' => Card { value: CardValue::Ten },
            b'J' => Card { value: CardValue::Jack },
            b'Q' => Card { value: CardValue::Queen },
            b'K' => Card { value: CardValue::King },
            b'A' => Card { value: CardValue::Ace },
            _ => panic!("WRONG CARD VALUE"),
        }
    }
}

#[derive(Debug, PartialEq,Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn new(cards: Vec<Card>) -> Self {
        Hand { cards }
    }
    fn evaluate(&self) -> HandType {
        // Sort the cards by value
        let mut sorted_cards = self.cards.clone();
        sorted_cards.sort();

        // Count occurrences of each card value
        let mut counts = HashMap::new();
        for card in &sorted_cards {
            *counts.entry(card.value).or_insert(0) += 1;
        }
        if counts.values().any(|&count| count == 5) {
            return HandType::FiveOfAKind;
        }
        if counts.values().any(|&count| count == 4) {
            return HandType::FourOfAKind;
        }
        if counts.values().any(|&count| count == 3) && counts.values().any(|&count| count == 2) {
            return HandType::FullHouse;
        }
        if counts.values().any(|&count| count == 3) {
            return HandType::ThreeOfAKind;
        }
        return match counts.iter().filter(|&(_, &count)| count == 2).collect::<Vec<_>>().len() {
            1 => HandType::OnePair,
            2 => HandType::TwoPair,
            _ => HandType::HighCard,
        }
    }
}



fn create_hand(input: &str) -> Hand {
    let mut result = Hand { cards: Vec::new() };
    for idx in 0..input.len() {
        let c = input.as_bytes()[idx];
        if c == b' ' {
            break;
        }
        result.cards.push(Card::new(c));
    }
    result
}

fn part1(input: &str) -> u64 {
    let mut hands: Vec<Hand> = input.lines().map(|x| create_hand(x)).collect();
    hands.sort_by(|a, b| a.evaluate().cmp(&b.evaluate()));
    0
    
}

fn main() {
    let input = include_str!("../exemple.txt");
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
