use crate::bullet::Bullet;
use crate::enemy::Enemy;
use crate::tower::Tower;
use crate::save_data::SaveData;
use rand::Rng;

fn distance(a: (f64, f64), b: (f64, f64)) -> f64 {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    (dx * dx + dy * dy).sqrt()
}

pub struct Game {
    enemies: Vec<Enemy>,
    bullets: Vec<Bullet>,
    score: u32,
    currency: u32,
    save_data: SaveData,
    time: f64,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    pub fn new() -> Self {
        let save_data = SaveData::load();
        Game {
            enemies: Vec::new(),
            bullets: Vec::new(),
            score: 0,
            currency: save_data.satoshis,
            save_data,
            time: 0.0,
        }
    }

    pub fn add_enemy(&mut self, enemy: Enemy) {
        self.enemies.push(enemy);
    }

    pub fn get_enemies(&self) -> &Vec<Enemy> {
        &self.enemies
    }

    pub fn get_bullets(&self) -> &Vec<Bullet> {
        &self.bullets
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }

    pub fn get_currency(&self) -> u32 {
        self.currency
    }

    pub fn get_high_score(&self) -> u32 {
        self.save_data.high_score
    }

    pub fn get_time(&self) -> f64 {
        self.time
    }

    pub fn increment_score(&mut self, points: u32) {
        self.score += points;
    }

    pub fn update(&mut self, tower: &mut Tower) {
        // Update time
        self.time += 1.0 / 60.0; // Assuming 60 FPS

        // Spawn enemies periodically
        if self.time % 3.0 < 1.0 / 60.0 { // Spawn every 3 seconds
            // Randomly choose spawn position from screen edges
            let spawn_pos = match rand::thread_rng().gen::<u8>() % 4 {
                0 => (0.0, rand::thread_rng().gen::<f64>() * 600.0), // Left edge
                1 => (800.0, rand::thread_rng().gen::<f64>() * 600.0), // Right edge
                2 => (rand::thread_rng().gen::<f64>() * 800.0, 0.0), // Top edge
                _ => (rand::thread_rng().gen::<f64>() * 800.0, 600.0), // Bottom edge
            };
            self.enemies.push(Enemy::new(100, 2.0, spawn_pos));
        }

        // Update enemies
        let tower_pos = tower.get_position();
        for enemy in &mut self.enemies {
            enemy.update_position(tower_pos);
        }

        // Update bullets and check for collisions
        let mut bullets_to_remove = Vec::new();
        let mut enemies_to_remove = Vec::new();

        for (bullet_idx, bullet) in self.bullets.iter_mut().enumerate() {
            bullet.update();

            // Remove bullets that have gone too far
            if bullet.get_distance_traveled() > 500.0 {
                bullets_to_remove.push(bullet_idx);
                continue;
            }

            // Check for collisions with enemies
            for (enemy_idx, enemy) in self.enemies.iter_mut().enumerate() {
                if bullet.check_collision(enemy.get_position()) {
                    enemy.take_damage(bullet.get_damage());
                    bullets_to_remove.push(bullet_idx);

                    // Check if enemy died
                    if enemy.get_health() == 0 {
                        enemies_to_remove.push(enemy_idx);
                        self.score += 10;
                        self.currency += 5;
                        self.save_data.satoshis = self.currency;
                        if self.score > self.save_data.high_score {
                            self.save_data.high_score = self.score;
                        }
                        // Save game data when currency changes
                        if let Err(e) = self.save_data.save() {
                            eprintln!("Failed to save game data: {}", e);
                        }
                    }
                    break;
                }
            }
        }

        // Remove dead enemies and collided bullets
        for idx in enemies_to_remove.iter().rev() {
            self.enemies.remove(*idx);
        }
        for idx in bullets_to_remove.iter().rev() {
            self.bullets.remove(*idx);
        }

        // Try to shoot at the nearest enemy
        if let Some(nearest_enemy) = self.find_nearest_enemy(tower_pos) {
            if let Some(bullet) = tower.shoot(nearest_enemy.get_position(), self.time) {
                self.bullets.push(bullet);
            }
        }
    }

    fn find_nearest_enemy(&self, tower_pos: (f64, f64)) -> Option<&Enemy> {
        self.enemies.iter().min_by(|a, b| {
            let dist_a = distance(tower_pos, a.get_position());
            let dist_b = distance(tower_pos, b.get_position());
            dist_a.partial_cmp(&dist_b).unwrap()
        })
    }

    pub fn save_game(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Update save data with current values
        self.save_data.save()
    }

    pub fn try_upgrade_tower_damage(&mut self, tower: &mut Tower) {
        let cost = tower.get_damage_upgrade_cost();
        if self.currency >= cost {
            self.currency -= cost;
            tower.upgrade_damage();
            self.save_data.satoshis = self.currency;
            self.save_data.damage_level = tower.get_damage_level();
            if let Err(e) = self.save_data.save() {
                eprintln!("Failed to save game after upgrade: {}", e);
            }
        }
    }

    pub fn try_upgrade_tower_fire_rate(&mut self, tower: &mut Tower) {
        let cost = tower.get_fire_rate_upgrade_cost();
        if self.currency >= cost {
            self.currency -= cost;
            tower.upgrade_fire_rate();
            self.save_data.satoshis = self.currency;
            self.save_data.fire_rate_level = tower.get_fire_rate_level();
            if let Err(e) = self.save_data.save() {
                eprintln!("Failed to save game after upgrade: {}", e);
            }
        }
    }
}
