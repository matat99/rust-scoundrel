# Scoundrel Card Game

A Rust implementation of the single-player roguelike card game "Scoundrel" by Zach Gage and Kurt Bieg.

## Project Structure

The game's code is organized into several modules, each handling specific aspects of the game:

### Core Files

- `main.rs`
  - The entry point of the application
  - Sets up the game instance and starts the main game loop
  - Contains module declarations and basic setup

- `game.rs`
  - Manages the overall game state and flow
  - Handles game loop logic
  - Coordinates between different game components (deck, player, room)
  - Contains the main turn logic and room management

### Game Components

- `card.rs`
  - Defines the card system fundamentals
  - Contains the `Card` struct and `CardType` enum
  - Implements card creation and deck management
  - Handles card display formatting
  - Includes the deck shuffling functionality

- `player.rs`
  - Manages player state (health, weapons, etc.)
  - Implements combat mechanics
  - Handles weapon management and monster tracking
  - Contains logic for health modifications and combat calculations

- `room.rs`
  - Implements the room system where cards are presented
  - Manages card selection and interaction
  - Handles room display and state
  - Controls the flow of card interactions within a room

### Utilities

- `utils.rs`
  - Contains shared utility functions
  - Handles user input processing
  - Houses common helper functions used across different modules

## Game Mechanics

The game implements the following core mechanics:
- Turn-based gameplay where players explore rooms with 4 cards each
- Combat system with weapon and barehanded fighting options
- Health and weapon management
- Monster difficulty scaling
- Room avoidance mechanics

## File Dependencies

- `main.rs` → Depends on `game.rs`
- `game.rs` → Depends on `card.rs`, `player.rs`, `room.rs`
- `player.rs` → Depends on `card.rs`, `utils.rs`
- `room.rs` → Depends on `card.rs`, `player.rs`
- All files may use `utils.rs` for common functionality

## Getting Started

1. Ensure you have Rust installed
2. Clone the repository
3. Run `cargo build` to compile
4. Run `cargo run` to start the game

## Game Rules
