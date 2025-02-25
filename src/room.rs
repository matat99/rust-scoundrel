// src/room.rs
use crate::cards::{Card, CardState, CardType};
use crate::player::{Player, STARTING_HEALTH};

#[derive(Debug)]
pub struct Room {
    pub cards: Vec<Card>,
    pub selected_count: u8,
    potion_used_this_turn: bool,
}

impl Room {
    pub fn new(cards: Vec<Card>) -> Self {
        Room {
            cards,
            selected_count: 0,
            potion_used_this_turn: false,
        }
    }

    pub fn display(&self) {
        println!("\n=== Current Room ===");
        println!("Cards remaining to select: {}", 3 - self.selected_count);
        print!("Cards: ");

        let active_cards: Vec<_> = self
            .cards
            .iter()
            .enumerate()
            .filter(|(_, card)| card.state == CardState::Active)
            .collect();

        for (i, (original_index, card)) in active_cards.iter().enumerate() {
            print!("{}. {}", original_index, card);
            if i < active_cards.len() - 1 {
                print!(" | ");
            }
        }
        println!("\n==================");
    }

    pub fn select_card(&mut self, index: usize, player: &mut Player) -> bool {
        if index >= self.cards.len() {
            println!(
                "Please select a number between 0 and {}",
                self.cards.len() - 1
            );
            return false;
        }

        if self.selected_count >= 3 {
            println!("Already selected 3 cards!");
            return false;
        }

        if self.cards[index].selected {
            println!("This card has already been selected!");
            return false;
        }

        self.cards[index].selected = true;

        match self.cards[index].card_type {
            CardType::Monster(_) => {
                player.fight_monster(&mut self.cards[index]); // Note: now mutable
                self.selected_count += 1;
                true
            }
            CardType::Weapon => {
                if let Some(_) = player.equipped_weapon.take() {
                    player.monsters_slain.clear();
                }
                self.cards[index].state = CardState::Used;
                player.equipped_weapon = Some(self.cards[index].clone());
                self.selected_count += 1;
                true
            }
            CardType::Potion => {
                if self.potion_used_this_turn {
                    println!("You've already used a potion this turn. This potion is discarded.");
                } else {
                    let heal_amount = self.cards[index].value;
                    player.health = (player.health + heal_amount).min(STARTING_HEALTH);
                    self.potion_used_this_turn = true;
                    println!(
                        "Used potion. Healed for {}. Health now: {}",
                        heal_amount, player.health
                    );
                }
                self.cards[index].state = CardState::Used;
                self.selected_count += 1;
                true
            }
        }
    }
}
