# Run Square Term

A simple terminal-based game written in Rust.

## Overview
This game is a terminal-based action game where you control a player character (`@`) on a grid. The objective is to collect coins (`$`) while avoiding enemies (`E`). Enemies spawn and become active after a short blinking period. The score increases as you collect coins. The game uses the `crossterm` crate for terminal manipulation and the `rand` crate for random number generation.

## Features
- Move your player around a 150x30 grid
- Collect coins to increase your score
- Avoid enemies that activate after spawning
- Enemies blink before becoming dangerous
- Real-time keyboard controls

## Controls
- Arrow keys: Move the player (`@`) up, down, left, or right
- `q`: Quit the game

## How to Run
1. **Install Rust**: If you don't have Rust installed, get it from [rustup.rs](https://rustup.rs/).
2. **Clone this repository** (if you haven't already):
   ```sh
   git clone <repo-url>
   cd run-square-term
   ```
3. **Run the game**:
   ```sh
   cargo run --release
   ```

## Dependencies
- [crossterm](https://crates.io/crates/crossterm) (for terminal manipulation)
- [rand](https://crates.io/crates/rand) (for random number generation)

## File Structure
- `src/main.rs`: Main game logic
- `Cargo.toml`: Project configuration and dependencies

## License
This project is licensed under the MIT License.

---

*Created: April 29, 2025*
