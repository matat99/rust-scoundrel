// src/game.rs
use crate::cards::{create_deck, Card, Shuffleable};
use crate::player::Player;
use crate::room::Room;
use std::io::{self, Write};

pub struct Game {
    deck: Vec<Card>,
    player: Player,
    can_skip: bool,
    previous_card: Option<Card>,
}

impl Game {
    pub fn get_deck_count(&self) -> usize {
        self.deck.len()
    }
    pub fn new() -> Self {
        let mut deck = create_deck();
        deck.shuffle();

        Game {
            deck,
            player: Player::new(),
            can_skip: true,
            previous_card: None,
        }
    }

    fn create_room(&mut self) -> Option<Room> {
        let mut room_cards = Vec::with_capacity(4);

        // First add the previous card if it exists
        if let Some(card) = self.previous_card.take() {
            room_cards.push(card);
        }

        // Draw remaining cards to fill the room
        while room_cards.len() < 4 {
            if let Some(card) = self.deck.pop() {
                room_cards.push(card);
            } else if room_cards.is_empty() {
                // If we have no cards at all, return None
                return None;
            } else {
                // If we have some cards but can't fill the room,
                // return what we have (end game scenario)
                return Some(Room::new(room_cards));
            }
        }

        Some(Room::new(room_cards))
    }

    pub fn run(&mut self) {
        'game: loop {
            match self.create_room() {
                Some(mut room) => 'room: loop {
                    println!("\nCards remaining in dungeon: {}", self.get_deck_count());
                    room.display();
                    self.player.display_status();
                    println!("\nOptions:");
                    println!("1. Face the room");
                    if self.can_skip {
                        println!("2. Avoid the room (place all cards at bottom of dungeon)");
                    }
                    print!("Choose: ");
                    io::stdout().flush().unwrap();

                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();

                    match input.trim() {
                        "2" if self.can_skip => {
                            // Take ownership of the cards from the room
                            let room_cards = std::mem::take(&mut room.cards);

                            // Insert the cards at the beginning of the deck (the bottom)
                            for card in room_cards {
                                self.deck.insert(0, card);
                            }

                            self.can_skip = false;
                            self.previous_card = None;
                            break 'room;
                        }
                        "1" => {
                            while room.selected_count < 3 {
                                println!("\nSelect a card (0-3) or 'q' to quit: ");
                                let mut input = String::new();
                                io::stdin().read_line(&mut input).unwrap();

                                let input = input.trim();
                                if input == "q" {
                                    break 'game;
                                }

                                if let Ok(index) = input.parse::<usize>() {
                                    if room.select_card(index, &mut self.player) {
                                        room.display();
                                        self.player.display_status();

                                        if self.player.health == 0 {
                                            // Optional: Calculate final score according to rules
                                            return; // Exit the run method entirely
                                        }
                                    }
                                } else {
                                    println!("Please enter a valid number!");
                                }
                            }

                            // Find the unselected card for the next room
                            self.previous_card =
                                room.cards.iter().find(|card| !card.selected).cloned();

                            self.can_skip = true;
                            break 'room;
                        }
                        _ => println!("Invalid choice!"),
                    }
                },
                None => break 'game,
            }
        }
    }
}
