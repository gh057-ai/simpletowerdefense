pub struct Enemy {
    health: u32,
    speed: u32,
}

impl Enemy {
    pub fn new(health: u32, speed: u32) -> Self {
        Enemy { health, speed }
    }

    pub fn take_damage(&mut self, damage: u32) {
        self.health = self.health.saturating_sub(damage);
    }

    pub fn get_health(&self) -> u32 {
        self.health
    }

    pub fn get_speed(&self) -> u32 {
        self.speed
    }
}
