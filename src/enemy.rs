pub struct Enemy {
    health: u32,
    max_health: u32,
    speed: f64,
    position: (f64, f64),
}

impl Enemy {
    pub fn new(health: u32, speed: f64, position: (f64, f64)) -> Self {
        Enemy {
            health,
            max_health: health,  // Store initial health as max_health
            speed,
            position,
        }
    }

    pub fn update_position(&mut self, target: (f64, f64)) {
        let dx = target.0 - self.position.0;
        let dy = target.1 - self.position.1;
        let distance = (dx * dx + dy * dy).sqrt();
        
        if distance > 0.0 {
            self.position.0 += (dx / distance) * self.speed;
            self.position.1 += (dy / distance) * self.speed;
        }
    }

    pub fn take_damage(&mut self, damage: u32) {
        self.health = self.health.saturating_sub(damage);
    }

    pub fn get_position(&self) -> (f64, f64) {
        self.position
    }

    pub fn get_health(&self) -> u32 {
        self.health
    }

    pub fn get_max_health(&self) -> u32 {
        self.max_health
    }

    pub fn get_health_percentage(&self) -> f64 {
        self.health as f64 / self.max_health as f64
    }
}
