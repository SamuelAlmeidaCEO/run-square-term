use std::time::Instant;

#[derive(Debug, Clone)]
pub struct Enemy {
    pub x: usize,
    pub y: usize,
    pub spawn_time: Instant,
    pub active: bool,
}

// Player could be a struct for future extensibility, but currently just coordinates.
pub type Player = (usize, usize);

// Coin is just a coordinate tuple.
pub type Coin = (usize, usize);
