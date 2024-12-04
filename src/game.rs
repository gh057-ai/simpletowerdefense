use crate::tower::Tower;
use crate::enemy::Enemy;

pub struct Game {
    score: u32,
    towers: Vec<Tower>,
    enemies: Vec<Enemy>,
    is_game_over: bool,
    is_game_paused: bool,
}

impl Game {
    pub fn new() -> Self {
        Game {
            score: 0,
            towers: vec![Tower::new(10, 100)], // Example tower
            enemies: vec![Enemy::new(100, 5)], // Example enemy
            is_game_over: false,
            is_game_paused: false,
        }
    }

    pub fn start_game(&mut self) {
        self.is_game_over = false;
        self.is_game_paused = false;
        // Add logic to initialize enemies and towers here
        let enemy = Enemy::new(100, 5); // Example enemy
        self.enemies.push(enemy);
        let tower = Tower::new(10, 100); // Example tower
        self.towers.push(tower);
    }

    pub fn pause_game(&mut self) {
        self.is_game_paused = true;
    }

    pub fn resume_game(&mut self) {
        self.is_game_paused = false;
    }

    pub fn end_game(&mut self) {
        self.is_game_over = true;
    }

    pub fn increment_score(&mut self, points: u32) {
        self.score += points;
    }
}
