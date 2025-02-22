// src/card.rs
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardType {
    Monster,
    Weapon,
    Potion,
}

#[derive(Debug, Clone)]
pub struct Card {
    pub card_type: CardType,
    pub value: u8,
    pub selected: bool,
}

pub trait Shuffleable {
    fn shuffle(&mut self);
}

impl Shuffleable for Vec<Card> {
    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.as_mut_slice().shuffle(&mut rng);
    }
}

pub fn create_deck() -> Vec<Card> {
    let mut deck = Vec::with_capacity(44);

    // Add Monsters (Clubs and Spades)
    for value in 2..=14 {
        for _ in 0..2 {
            deck.push(Card {
                card_type: CardType::Monster,
                value,
                selected: false,
            });
        }
    }

    // Add Potions (Hearts)
    for value in 2..=10 {
        deck.push(Card {
            card_type: CardType::Potion,
            value,
            selected: false,
        });
    }

    // Add Weapons (Diamonds)
    for value in 2..=10 {
        deck.push(Card {
            card_type: CardType::Weapon,
            value,
            selected: false,
        });
    }

    deck
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value_str = match self.value {
            1 => "A".to_string(),
            11 => "J".to_string(),
            12 => "Q".to_string(),
            13 => "K".to_string(),
            14 => "A".to_string(),
            n => n.to_string(),
        };

        match self.card_type {
            CardType::Monster => write!(f, "{}♠/♣", value_str),
            CardType::Weapon => write!(f, "{}♦", value_str),
            CardType::Potion => write!(f, "{}♥", value_str),
        }
    }
}
