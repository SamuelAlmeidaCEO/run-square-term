use crate::entities::{Enemy, Coin};
use crate::ui::{WIDTH, HEIGHT};
use std::time::{Duration, Instant};
use rand::Rng;

pub fn update_enemy_activation(enemies: &mut Vec<Enemy>, now: Instant) {
    for enemy in enemies.iter_mut() {
        if !enemy.active && now.duration_since(enemy.spawn_time) > Duration::from_secs(5) {
            enemy.active = true;
        }
    }
}

pub fn move_enemies(enemies: &mut Vec<Enemy>, player_x: usize, player_y: usize) {
    for enemy in enemies.iter_mut() {
        if enemy.active {
            if enemy.x < player_x { enemy.x += 1; }
            else if enemy.x > player_x { enemy.x -= 1; }
            if enemy.y < player_y { enemy.y += 1; }
            else if enemy.y > player_y { enemy.y -= 1; }
        }
    }
}

pub fn spawn_enemy(enemies: &mut Vec<Enemy>, now: Instant) {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0..WIDTH);
    let y = rng.gen_range(0..HEIGHT);
    enemies.push(Enemy { x, y, spawn_time: now, active: false });
}

pub fn spawn_coin(coins: &mut Vec<Coin>, enemies: &Vec<Enemy>) {
    let mut rng = rand::thread_rng();
    loop {
        let x = rng.gen_range(0..WIDTH);
        let y = rng.gen_range(0..HEIGHT);
        if !enemies.iter().any(|e| e.x == x && e.y == y) && !coins.contains(&(x, y)) {
            coins.push((x, y));
            break;
        }
    }
}

pub fn check_coin_collection(player_x: usize, player_y: usize, coins: &mut Vec<Coin>) -> bool {
    if let Some(idx) = coins.iter().position(|&(x, y)| x == player_x && y == player_y) {
        coins.remove(idx);
        true
    } else {
        false
    }
}

pub fn check_enemy_collision(player_x: usize, player_y: usize, enemies: &Vec<Enemy>) -> bool {
    enemies.iter().any(|e| e.x == player_x && e.y == player_y && e.active)
}
