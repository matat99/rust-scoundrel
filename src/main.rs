// src/main.rs
mod cards;
mod game;
mod player;
mod room;
mod utils;

use game::Game;
use utils::get_user_input;

fn display_rules() {
    println!("\n=== SCOUNDREL - Game Rules ===");
    println!("\nIf you'd like to see the original rules look at the README.md file");
    println!("\nSetup:");
    println!("• Started with 20 Health");
    println!("• Uses standard deck minus Red Face Cards and Red Aces");

    println!("\nCard Types:");
    println!("• Monsters (Clubs & Spades) - Deal damage equal to their value");
    println!("  (Jack=11, Queen=12, King=13, Ace=14)");
    println!("• Weapons (Diamonds) - Reduce monster damage by their value");
    println!("• Health Potions (Hearts) - Restore health equal to their value");

    println!("\nGameplay:");
    println!("• Each turn reveals 4 cards forming a Room");
    println!("• Must face 3 of the 4 cards each turn");
    println!("• Can skip a room (but not two in a row)");
    println!("• Game ends when life reaches 0 or deck is empty");

    println!("\nCombat Rules:");
    println!("• Can fight monsters barehanded (take full damage)");
    println!("• Weapons reduce monster damage by weapon value");
    println!("• Weapons can only fight monsters weaker than the last monster they killed");

    println!("\nPress Enter to return to menu...");
    get_user_input();
}

fn run_start_menu() -> bool {
    loop {
        println!("\n=== SCOUNDREL ===");
        println!("1. Start Game");
        println!("2. View Rules");
        println!("q. Quit");
        println!("\nEnter your choice: ");

        match get_user_input().as_str() {
            "1" => return true,
            "2" => display_rules(),
            "q" => return false,
            _ => println!("Invalid choice! Please try again."),
        }
    }
}

fn main() {
    while run_start_menu() {
        let mut game = Game::new();
        game.run();

        println!("\nGame Over!");
        println!("Press Enter to return to menu...");
        get_user_input();
    }

    println!("\nSee Ya!");
}
