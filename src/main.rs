// src/main.rs
mod cards;
mod game;
mod player;
mod room;
mod utils;

use game::Game;

fn main() {
    let mut game = Game::new();
    game.run();
}
