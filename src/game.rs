// src/game.rs
use crate::cards::{create_deck, Card, Shuffleable};
use crate::player::Player;
use crate::room::Room;
use std::io::{self, Write};

pub struct Game {
    deck: Vec<Card>,
    player: Player,
    discard: Vec<Card>,
    can_skip: bool,
    previous_card: Option<Card>,
}

impl Game {
    pub fn new() -> Self {
        let mut deck = create_deck();
        deck.shuffle();

        Game {
            deck,
            player: Player::new(),
            discard: Vec::new(),
            can_skip: true,
            previous_card: None,
        }
    }

    fn create_room(&mut self) -> Option<Room> {
        let mut room_cards = Vec::with_capacity(4);

        if let Some(card) = self.previous_card.take() {
            room_cards.push(card);
        }

        while room_cards.len() < 4 {
            if let Some(card) = self.deck.pop() {
                room_cards.push(card);
            } else {
                return None;
            }
        }

        Some(Room::new(room_cards))
    }

    pub fn run(&mut self) {
        'game: loop {
            match self.create_room() {
                Some(mut room) => 'room: loop {
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
                            self.deck.extend(room.cards);
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
                                    }
                                } else {
                                    println!("Please enter a valid number!");
                                }
                            }

                            self.can_skip = true;
                            self.previous_card = Some(room.cards[3].clone());
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
