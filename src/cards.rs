// src/cards.rs
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonsterSuit {
    Spade,
    Club,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardState {
    Active,
    Defeated,
    Used,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardType {
    Monster(MonsterSuit), // Can only be Spade or Club
    Weapon,               // Always Diamond
    Potion,               // Always Heart
}

#[derive(Debug, Clone)]
pub struct Card {
    pub card_type: CardType,
    pub value: u8,
    pub selected: bool,
    pub state: CardState,
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

    // Add Spades
    for value in 2..=14 {
        deck.push(Card {
            card_type: CardType::Monster(MonsterSuit::Spade),
            value,
            selected: false,
            state: CardState::Active,
        });
    }

    // Add Clubs
    for value in 2..=14 {
        deck.push(Card {
            card_type: CardType::Monster(MonsterSuit::Club),
            value,
            selected: false,
            state: CardState::Active,
        });
    }

    // Add Potions (Hearts)
    for value in 2..=10 {
        deck.push(Card {
            card_type: CardType::Potion,
            value,
            selected: false,
            state: CardState::Active,
        });
    }

    // Add Weapons (Diamonds)
    for value in 2..=10 {
        deck.push(Card {
            card_type: CardType::Weapon,
            value,
            selected: false,
            state: CardState::Active,
        });
    }

    deck
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value_str = match self.value {
            11 => "J".to_string(),
            12 => "Q".to_string(),
            13 => "K".to_string(),
            14 => "A".to_string(),
            n => n.to_string(),
        };

        match self.card_type {
            CardType::Monster(MonsterSuit::Spade) => write!(f, "{}♠", value_str),
            CardType::Monster(MonsterSuit::Club) => write!(f, "{}♣", value_str),
            CardType::Weapon => write!(f, "{}♦", value_str),
            CardType::Potion => write!(f, "{}♥", value_str),
        }
    }
}
