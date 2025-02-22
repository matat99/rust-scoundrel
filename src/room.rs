// src/room.rs
use crate::cards::{Card, CardType};
use crate::player::{Player, STARTING_HEALTH};

#[derive(Debug)]
pub struct Room {
    pub cards: Vec<Card>,
    pub selected_count: u8,
}

impl Room {
    pub fn new(cards: Vec<Card>) -> Self {
        Room {
            cards,
            selected_count: 0,
        }
    }

    pub fn display(&self) {
        println!("\n=== Current Room ===");
        println!("Cards remaining to select: {}", 3 - self.selected_count);
        for (i, card) in self.cards.iter().enumerate() {
            let status = if card.selected {
                "[Selected]"
            } else if card.card_type == CardType::Monster {
                "[Must Fight]"
            } else {
                "[Available]"
            };
            println!("{}. {} {}", i, card, status);
        }
        println!("==================");
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

        match self.cards[index].card_type {
            CardType::Monster => {
                player.fight_monster(&self.cards[index]);
                self.cards[index].selected = true;
                self.selected_count += 1;
                true
            }
            CardType::Weapon => {
                if let Some(_) = player.equipped_weapon.take() {
                    player.monsters_slain.clear();
                }
                player.equipped_weapon = Some(self.cards[index].clone());
                self.cards[index].selected = true;
                self.selected_count += 1;
                true
            }
            CardType::Potion => {
                let heal_amount = self.cards[index].value;
                player.health = (player.health + heal_amount).min(STARTING_HEALTH);
                self.cards[index].selected = true;
                self.selected_count += 1;
                true
            }
        }
    }
}
