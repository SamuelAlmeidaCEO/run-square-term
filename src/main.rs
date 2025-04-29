use crossterm::{cursor, event, terminal, ExecutableCommand};
use crossterm::event::{read, Event, KeyCode, KeyEvent};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use std::io::stdout;
use std::time::{Duration, Instant};

mod entities;
mod ui;
mod game;

use entities::{Enemy, Coin};
use ui::{draw, WIDTH, HEIGHT};
use game::{update_enemy_activation, move_enemies, spawn_enemy, spawn_coin, check_coin_collection};

fn main() {
    let mut stdout = stdout();
    terminal::enable_raw_mode().unwrap();
    stdout.execute(EnterAlternateScreen).unwrap();
    stdout.execute(cursor::Hide).unwrap();

    let (mut x, mut y) = (WIDTH / 2, HEIGHT / 2);
    let mut enemies: Vec<Enemy> = vec![Enemy { x: 1, y: 1, spawn_time: Instant::now(), active: false }];
    let mut last_enemy_move = Instant::now();
    let enemy_move_interval = Duration::from_millis(300);
    let mut last_enemy_spawn = Instant::now();
    let enemy_spawn_interval = Duration::from_secs(15);
    let mut coins: Vec<Coin> = Vec::new();
    let mut last_coin_spawn = Instant::now();
    let coin_spawn_interval = Duration::from_secs(10);
    let mut score: u32 = 0;
    let start_time = Instant::now();
    draw(x, y, &enemies, &coins, score, start_time);

    'game: loop {
        let now = Instant::now();
        // Handle player input
        if event::poll(Duration::from_millis(10)).unwrap() {
            if let Event::Key(KeyEvent { code, modifiers: _, .. }) = read().unwrap() {
                match code {
                    KeyCode::Char('w') => if y > 0 { y -= 1; },
                    KeyCode::Char('a') => if x > 0 { x -= 1; },
                    KeyCode::Char('s') => if y < HEIGHT - 1 { y += 1; },
                    KeyCode::Char('d') => if x < WIDTH - 1 { x += 1; },
                    KeyCode::Char('q') => break 'game,
                    _ => {}
                }
            }
        }
        // Move all active enemies toward player only if enough time has passed
        if last_enemy_move.elapsed() >= enemy_move_interval {
            update_enemy_activation(&mut enemies, now);
            move_enemies(&mut enemies, x, y);
            last_enemy_move = Instant::now();
        }
        // Spawn a new enemy every 15 seconds
        if last_enemy_spawn.elapsed() >= enemy_spawn_interval {
            spawn_enemy(&mut enemies, now);
            last_enemy_spawn = Instant::now();
        }
        // Spawn a coin every 10 seconds at a random empty location
        if last_coin_spawn.elapsed() >= coin_spawn_interval {
            spawn_coin(&mut coins, &enemies);
            last_coin_spawn = Instant::now();
        }
        // Player collects coin
        if check_coin_collection(x, y, &mut coins) {
            score += 1;
        }
        draw(x, y, &enemies, &coins, score, now);
        // End game if player collides with any active enemy
        if enemies.iter().any(|e| e.active && e.x == x && e.y == y) {
            break 'game;
        }
    }

    // After the game, show the cursor and disable raw mode
    stdout.execute(cursor::Show).unwrap();
    stdout.execute(LeaveAlternateScreen).unwrap();
    terminal::disable_raw_mode().unwrap();
    // Print exit message below the grid
    println!("\nGame Over! The enemy caught you. Final score: {}. Goodbye!", score);
}
