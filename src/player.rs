// src/player.rs
use crate::cards::{Card, CardType};
use crate::utils::get_user_input;

pub const STARTING_HEALTH: u8 = 20;

#[derive(Debug)]
pub struct Player {
    pub health: u8,
    pub equipped_weapon: Option<Card>,
    pub monsters_slain: Vec<Card>,
}

impl Player {
    pub fn new() -> Self {
        Player {
            health: STARTING_HEALTH,
            equipped_weapon: None,
            monsters_slain: Vec::new(),
        }
    }

    pub fn display_status(&self) {
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

    pub fn can_use_weapon_against(&self, monster_value: u8) -> bool {
        match &self.equipped_weapon {
            Some(_) => {
                if self.monsters_slain.is_empty() {
                    true
                } else {
                    let last_monster = self.monsters_slain.last().unwrap();
                    monster_value <= last_monster.value
                }
            }
            None => false,
        }
    }

    pub fn fight_monster(&mut self, monster: &Card) {
        println!("\nFighting {}!", monster);
        if self.equipped_weapon.is_some() && self.can_use_weapon_against(monster.value) {
            println!("Options:");
            println!("1. Fight barehanded");
            println!("2. Use equipped weapon");
            print!("Choose: ");

            match get_user_input().as_str() {
                "2" => self.fight_with_weapon(monster),
                _ => self.fight_barehanded(monster),
            }
        } else {
            println!("Fighting barehanded (no usable weapon available)");
            self.fight_barehanded(monster);
        }
    }

    fn fight_barehanded(&mut self, monster: &Card) {
        println!("Fighting {} barehanded!", monster);
        if self.health >= monster.value {
            self.health -= monster.value;
            println!(
                "Took {} damage. Health remaining: {}",
                monster.value, self.health
            );
        } else {
            self.health = 0;
            println!("You have been defeated!");
        }
    }

    fn fight_with_weapon(&mut self, monster: &Card) {
        if let Some(weapon) = &self.equipped_weapon {
            println!("Fighting {} with weapon {}", monster, weapon);
            let damage = if monster.value > weapon.value {
                monster.value - weapon.value
            } else {
                0
            };

            if self.health >= damage {
                self.health -= damage;
                println!("Took {} damage. Health remaining: {}", damage, self.health);
                self.monsters_slain.push(monster.clone());
            } else {
                self.health = 0;
                println!("You have been defeated!");
            }
        }
    }
}
