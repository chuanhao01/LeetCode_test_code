use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::File,
    io::{Read, Result},
};

use itertools::izip;

#[derive(Debug)]
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
            _type: Type::new(&cards),
        }
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self._type.cmp(&other._type) {
            Ordering::Equal => {
                for (card1, card2) in izip!(&self.cards, &other.cards) {
                    match card1.cmp(card2) {
                        Ordering::Equal => {
                            continue;
                        }
                        order => return order,
                    }
                }
                Ordering::Equal
            }
            order => order,
        }
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Eq for Hand {}
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        let mut cards1 = self.cards.clone();
        cards1.sort();
        let mut cards2 = other.cards.clone();
        cards2.sort();
        izip!(cards1, cards2).all(|(card1, card2)| card1 == card2)
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
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
    pub fn new(cards: &Vec<Card>) -> Self {
        let joker_cards = cards.iter().filter(|card| **card == Card::J).count();
        if joker_cards > 0 {
            let mut other_card_type = Self::from(
                &cards
                    .clone()
                    .into_iter()
                    .filter(|card| *card != Card::J)
                    .collect::<Vec<_>>(),
            );
            // println!("cards: {:?}", cards);
            // println!(
            //     "filter_cards: {:?}",
            //     &cards
            //         .clone()
            //         .into_iter()
            //         .filter(|card| *card != Card::J)
            //         .collect::<Vec<_>>(),
            // );
            // println!("joker_cards: {}", joker_cards);
            // println!("other_card_type: {:?}", other_card_type);
            for _ in 0..joker_cards {
                other_card_type = Self::joker_upgrade(other_card_type);
            }
            println!("new_card_type: {:?}", other_card_type);
            other_card_type
        } else {
            Self::from(cards)
        }
    }
    fn joker_upgrade(current_type: Type) -> Type {
        match current_type {
            Self::HighCard => Self::OnePair,
            Self::OnePair => Self::ThreeOfAKind,
            Self::ThreeOfAKind => Self::FourOfAKind,
            Self::FourOfAKind => Self::FiveOfAKind,
            Self::TwoPair => Self::FullHouse,
            Self::FullHouse => Self::FourOfAKind,
            // Solve bug with 5 jokers
            Self::FiveOfAKind => Self::FiveOfAKind,
        }
    }
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

#[derive(PartialEq, PartialOrd, Ord, Clone, Copy, Eq, Hash, Debug)]
enum Card {
    J,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
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
    let mut file_input = File::open("inputs/d7")?;
    // let mut file_input = File::open("inputs/temp")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;
    let input = input.split('\n');
    let mut hands = input
        .map(|raw_hand| {
            let raw_hand = raw_hand.split(' ').collect::<Vec<_>>();
            let raw_cards = raw_hand[0];
            let bid = raw_hand[1].parse::<i64>().unwrap();
            Hand::new(raw_cards.to_string(), bid)
        })
        .collect::<Vec<_>>();
    hands.sort();
    println!("{:?}", hands);

    let mut sum = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + (i as i64 + 1) * hand.bid);
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
            Type::from(&Card::from_cards(&String::from("AAAAA"))),
            Type::FiveOfAKind
        ));
        assert!(matches!(
            Type::from(&Card::from_cards(&String::from("AA8AA"))),
            Type::FourOfAKind
        ));
        assert!(matches!(
            Type::from(&Card::from_cards(&String::from("23332"))),
            Type::FullHouse
        ));
        assert!(matches!(
            Type::from(&Card::from_cards(&String::from("TTT98"))),
            Type::ThreeOfAKind
        ));
        assert!(matches!(
            Type::from(&Card::from_cards(&String::from("23432"))),
            Type::TwoPair
        ));
        assert!(matches!(
            Type::from(&Card::from_cards(&String::from("A23A4"))),
            Type::OnePair
        ));
        assert!(matches!(
            Type::from(&Card::from_cards(&String::from("23456"))),
            Type::HighCard
        ));
    }
}
