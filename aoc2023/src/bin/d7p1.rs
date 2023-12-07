use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::File,
    io::{Read, Result},
};

use itertools::izip;

struct Hand {
    cards: Vec<Card>,
    bid: i64,
    _type: Type,
}
impl Hand {
    pub fn new(raw_cards: String, bid: i64) -> Self {
        let cards = Card::from_cards(&raw_cards);
        Self {
            cards: cards.clone(),
            bid,
            _type: Type::from(&cards),
        }
    }
}

#[derive(PartialEq, PartialOrd)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}
impl Type {
    pub fn from(cards: &Vec<Card>) -> Self {
        let mut card_counts: HashMap<Card, i64> = HashMap::new();
        for card in cards {
            card_counts
                .entry(*card)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        for (_, &counts) in card_counts.iter() {
            if counts == 5 {
                return Type::FiveOfAKind;
            }
            if counts == 4 {
                return Type::FourOfAKind;
            }
        }
        for (_, &counts) in card_counts.iter() {
            if counts == 3 {
                for (_, &counts) in card_counts.iter() {
                    if counts == 2 {
                        return Type::FullHouse;
                    }
                }
                return Type::ThreeOfAKind;
            }
        }
        let pair_counts = card_counts.iter().fold(
            0,
            |acc, (_, &counts)| {
                if counts == 2 {
                    acc + 1
                } else {
                    acc
                }
            },
        );
        if pair_counts == 2 {
            Type::TwoPair
        } else if pair_counts == 1 {
            Type::OnePair
        } else {
            Type::HighCard
        }
    }
}

#[derive(PartialEq, PartialOrd, Clone, Copy, Eq, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}
impl Card {
    pub fn from(raw_card: &str) -> Self {
        if raw_card == "A" {
            Card::A
        } else if raw_card == "K" {
            Card::K
        } else if raw_card == "Q" {
            Card::Q
        } else if raw_card == "J" {
            Card::J
        } else if raw_card == "T" {
            Card::T
        } else if raw_card == "9" {
            Card::Nine
        } else if raw_card == "8" {
            Card::Eight
        } else if raw_card == "7" {
            Card::Seven
        } else if raw_card == "6" {
            Card::Six
        } else if raw_card == "5" {
            Card::Five
        } else if raw_card == "4" {
            Card::Four
        } else if raw_card == "3" {
            Card::Three
        } else if raw_card == "2" {
            Card::Two
        } else {
            panic!("Wrong Input, Card not expected. Card: {}", raw_card);
        }
    }
    pub fn from_cards(raw_cards: &str) -> Vec<Self> {
        let mut cards: Vec<Card> = Vec::new();
        for raw_card in raw_cards.chars() {
            let raw_card = raw_card.to_string();
            cards.push(Self::from(&raw_card));
        }
        cards
    }
}

fn main() -> Result<()> {
    let mut file_input = File::open("inputs/d6")?;
    // let mut file_input = File::open("inputs/temp")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;

    let mut sum = 0;
    println!("sum: {}", sum);
    Ok(())
}

// Write test for get type for cards
// Check for each type test case
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_card_get_type() {
        assert!(matches!(
            Hand::get_type(&String::from("AAAAA")),
            Type::FiveOfAKind
        ));
        assert!(matches!(
            Hand::get_type(&String::from("AA8AA")),
            Type::FourOfAKind
        ));
        assert!(matches!(
            Hand::get_type(&String::from("23332")),
            Type::FullHouse
        ));
        assert!(matches!(
            Hand::get_type(&String::from("TTT98")),
            Type::ThreeOfAKind
        ));
        assert!(matches!(
            Hand::get_type(&String::from("23432")),
            Type::TwoPair
        ));
        assert!(matches!(
            Hand::get_type(&String::from("A23A4")),
            Type::OnePair
        ));
        assert!(matches!(
            Hand::get_type(&String::from("23456")),
            Type::HighCard
        ));
    }
}
