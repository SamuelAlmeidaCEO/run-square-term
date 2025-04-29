use std::io::{stdout, Write};
use crossterm::{cursor, ExecutableCommand};
use crate::entities::{Enemy, Coin};
use std::time::Instant;

pub const WIDTH: usize = 150;
pub const HEIGHT: usize = 30;

pub fn draw(player_x: usize, player_y: usize, enemies: &Vec<Enemy>, coins: &Vec<Coin>, score: u32, now: Instant) {
    let mut stdout = stdout();
    stdout.execute(cursor::MoveTo(0, 0)).unwrap();

    let mut buffer = String::new();
    buffer.push_str(&format!("Score: {}\n", score));
    buffer.push('+');
    for _ in 0..WIDTH { buffer.push('-'); }
    buffer.push('+');
    buffer.push('\n');
    for y in 0..HEIGHT {
        buffer.push('|');
        for x in 0..WIDTH {
            if x == player_x && y == player_y {
                buffer.push('@');
            } else if let Some(enemy) = enemies.iter().find(|e| e.x == x && e.y == y) {
                let blink_time = now.duration_since(enemy.spawn_time);
                if !enemy.active {
                    // Blink for 5s, visible for 250ms every 500ms
                    if (blink_time.as_millis() / 250) % 2 == 0 {
                        buffer.push('e');
                    } else {
                        buffer.push('.');
                    }
                } else {
                    buffer.push('E');
                }
            } else if coins.contains(&(x, y)) {
                buffer.push('$');
            } else {
                buffer.push('.');
            }
        }
        buffer.push('|');
        buffer.push('\n');
    }
    buffer.push('+');
    for _ in 0..WIDTH { buffer.push('-'); }
    buffer.push('+');
    buffer.push('\n');
    let formatted = buffer.replace('\n', "\r\n");
    stdout.write_all(formatted.as_bytes()).unwrap();
    stdout.flush().unwrap();
}
