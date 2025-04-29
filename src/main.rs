use crossterm::{cursor, event, terminal, ExecutableCommand};
use crossterm::event::{read, Event, KeyCode, KeyEvent};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use std::io::stdout;
use std::time::{Duration, Instant};
use rand::Rng;

struct Enemy {
    x: usize,
    y: usize,
    spawn_time: Instant,
    active: bool,
}

const WIDTH: usize = 150;
const HEIGHT: usize = 30;

fn draw(player_x: usize, player_y: usize, enemies: &Vec<Enemy>, coins: &Vec<(usize, usize)>, score: u32, now: Instant) {
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
    use std::io::Write;
    stdout.write_all(formatted.as_bytes()).unwrap();
    stdout.flush().unwrap();
}



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
    let mut coins: Vec<(usize, usize)> = Vec::new();
    let mut last_coin_spawn = Instant::now();
    let coin_spawn_interval = Duration::from_secs(10);
    let mut rng = rand::thread_rng();
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
            for enemy in enemies.iter_mut() {
                let blink_time = now.duration_since(enemy.spawn_time);
                if blink_time >= Duration::from_secs(5) {
                    enemy.active = true;
                }
                if enemy.active {
                    if enemy.x < x { enemy.x += 1; }
                    else if enemy.x > x { enemy.x -= 1; }
                    if enemy.y < y { enemy.y += 1; }
                    else if enemy.y > y { enemy.y -= 1; }
                }
            }
            last_enemy_move = Instant::now();
        }
        // Spawn a new enemy every 15 seconds
        if last_enemy_spawn.elapsed() >= enemy_spawn_interval {
            let mut attempts = 0;
            loop {
                let ex = rng.gen_range(0..WIDTH);
                let ey = rng.gen_range(0..HEIGHT);
                // Only spawn if not on player, coins, or another enemy
                if (ex != x || ey != y)
                    && !coins.contains(&(ex, ey))
                    && !enemies.iter().any(|e| e.x == ex && e.y == ey) {
                    enemies.push(Enemy { x: ex, y: ey, spawn_time: now, active: false });
                    break;
                }
                attempts += 1;
                if attempts > 1000 { break; }
            }
            last_enemy_spawn = Instant::now();
        }
        // Spawn a coin every 10 seconds at a random empty location
        if last_coin_spawn.elapsed() >= coin_spawn_interval {
            let mut attempts = 0;
            loop {
                let coin_x = rng.gen_range(0..WIDTH);
                let coin_y = rng.gen_range(0..HEIGHT);
                // Only spawn if not on player, any enemy, or another coin
                if (coin_x != x || coin_y != y)
                    && !enemies.iter().any(|e| e.x == coin_x && e.y == coin_y)
                    && !coins.contains(&(coin_x, coin_y)) {
                    coins.push((coin_x, coin_y));
                    break;
                }
                attempts += 1;
                if attempts > 1000 { break; }
            }
            last_coin_spawn = Instant::now();
        }
        // Player collects coin
        if let Some(idx) = coins.iter().position(|&(cx, cy)| cx == x && cy == y) {
            coins.remove(idx);
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


