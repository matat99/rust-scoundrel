#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CardType {
    Monster,
    Weapon,
    Potion,
}

#[derive(Debug)]
struct Card {
    card_type: CardType,
    value: u8,
}

struct Player {
    cards: Vec<Card>,
    health: u8,
}

fn create_deck() -> Vec<Card> {
    let mut deck = Vec::with_capacity(52);
    for value in 1..=14 {
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
    for value in 1..=10 {
        deck.push(Card {
            card_type: CardType::Weapon,
            value,
        });
    }
    deck
}

fn main() {
    let deck = create_deck();
    println!("{:?}", deck);
    println!("deck size: {}", deck.len());
}
