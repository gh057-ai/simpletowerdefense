pub struct Bullet {
    position: (f64, f64),
    velocity: (f64, f64),
    damage: u32,
    start_position: (f64, f64),
}

impl Bullet {
    pub fn new(start: (f64, f64), target: (f64, f64), speed: f64, damage: u32) -> Self {
        let dx = target.0 - start.0;
        let dy = target.1 - start.1;
        let distance = (dx * dx + dy * dy).sqrt();
        
        let velocity = if distance > 0.0 {
            ((dx / distance) * speed, (dy / distance) * speed)
        } else {
            (0.0, 0.0)
        };

        Bullet {
            position: start,
            velocity,
            damage,
            start_position: start,
        }
    }

    pub fn update(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
    }

    pub fn get_position(&self) -> (f64, f64) {
        self.position
    }

    pub fn get_damage(&self) -> u32 {
        self.damage
    }

    pub fn get_distance_traveled(&self) -> f64 {
        let dx = self.position.0 - self.start_position.0;
        let dy = self.position.1 - self.start_position.1;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn check_collision(&self, target_pos: (f64, f64)) -> bool {
        let dx = self.position.0 - target_pos.0;
        let dy = self.position.1 - target_pos.1;
        let distance_squared = dx * dx + dy * dy;
        distance_squared < 20.0 * 20.0 // Collision within 20 pixels
    }
}
