use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CardType {
    Monster,
    Weapon,
    Potion,
}

#[derive(Debug, Clone)]
struct Room {
    cards: Vec<Card>,
}

trait Shuffleable {
    fn shuffle(&mut self);
}

impl Shuffleable for Vec<Card> {
    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.as_mut_slice().shuffle(&mut rng); // Convert Vec to slice first
    }
}

#[derive(Debug, Clone)]
struct Card {
    card_type: CardType,
    value: u8,
}

struct Player {
    health: u8,
    equipped_weapon: Option<Card>,
    monsters_slain: Vec<Card>, // Monsters slain by current weapon
}

const STARTING_HEALTH: u8 = 20;

impl Player {
    fn new() -> Self {
        Player {
            health: STARTING_HEALTH,
            equipped_weapon: None,
            monsters_slain: Vec::new(),
        }
    }

    fn display_status(&self) {
        println!("\nHealth: {}", self.health);
        match &self.equipped_weapon {
            Some(weapon) => {
                println!("Equipped weapon: {}", weapon);
                if !self.monsters_slain.is_empty() {
                    println!("Monsters slain by this weapon:");
                    for monster in &self.monsters_slain {
                        println!("  {}", monster);
                    }
                }
            }
            None => println!("No weapon equipped"),
        }
    }
}

// Add display formatting for cards
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
            CardType::Monster => write!(f, "{}♠ /♣", value_str),
            CardType::Weapon => write!(f, "{}♦", value_str),
            CardType::Potion => write!(f, "{}♥", value_str),
        }
    }
}

fn create_deck() -> Vec<Card> {
    let mut deck = Vec::with_capacity(44);
    for value in 2..=14 {
        deck.push(Card {
            card_type: CardType::Monster,
            value,
        });
        deck.push(Card {
            card_type: CardType::Monster,
            value,
        });
    }
    for value in 2..=10 {
        deck.push(Card {
            card_type: CardType::Potion,
            value,
        });
    }
    for value in 2..=10 {
        deck.push(Card {
            card_type: CardType::Weapon,
            value,
        });
    }
    deck
}

impl Room {
    fn new(cards: Vec<Card>) -> Self {
        Room { cards }
    }

    fn display(&self) {
        println!("\nCurrent Room:");
        for (i, card) in self.cards.iter().enumerate() {
            println!("{}. {}", i + 1, card);
        }
    }
}

fn create_room(deck: &mut Vec<Card>, previous_card: Option<Card>) -> Option<Room> {
    let mut room_cards = Vec::with_capacity(4);

    // Add the previous card if it exists
    if let Some(card) = previous_card {
        room_cards.push(card);
    }

    // Draw until we have 4 cards
    while room_cards.len() < 4 {
        if let Some(card) = deck.pop() {
            room_cards.push(card);
        } else {
            return None; // Dungeon is empty
        }
    }

    Some(Room::new(room_cards))
}

fn main() {
    let mut deck = create_deck();
    deck.shuffle();
    let mut player = Player::new();
    let mut discard: Vec<Card> = Vec::new(); // Specify the type

    if let Some(room) = create_room(&mut deck, None) {
        room.display();
        player.display_status();
        println!("\nOptions:");
        println!("1. Face the room");
        println!("2. Avoid the room (place all cards at bottom of dungeon)");
    }
}
