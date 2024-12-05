use crate::bullet::Bullet;

pub struct Tower {
    damage: u32,
    range: u32,
    position: (f64, f64),
    shoot_cooldown: f64,
    last_shot: f64,
    damage_level: u32,
    fire_rate_level: u32,
}

impl Tower {
    pub fn new(damage: u32, range: u32, position: (f64, f64)) -> Self {
        Tower {
            damage,
            range,
            position,
            shoot_cooldown: 0.5,
            last_shot: 0.0,
            damage_level: 1,
            fire_rate_level: 1,
        }
    }

    pub fn upgrade_damage(&mut self) {
        self.damage_level += 1;
        self.damage += 5; // Each upgrade adds 5 damage
    }

    pub fn upgrade_fire_rate(&mut self) {
        self.fire_rate_level += 1;
        self.shoot_cooldown *= 0.8; // Each upgrade reduces cooldown by 20%
    }

    pub fn get_damage_upgrade_cost(&self) -> u32 {
        20 * self.damage_level // Cost increases with level
    }

    pub fn get_fire_rate_upgrade_cost(&self) -> u32 {
        25 * self.fire_rate_level // Cost increases with level
    }

    pub fn get_damage(&self) -> u32 {
        self.damage
    }

    pub fn get_range(&self) -> u32 {
        self.range
    }

    pub fn get_position(&self) -> (f64, f64) {
        self.position
    }

    pub fn get_damage_level(&self) -> u32 {
        self.damage_level
    }

    pub fn get_fire_rate_level(&self) -> u32 {
        self.fire_rate_level
    }

    pub fn shoot(&mut self, target_position: (f64, f64), current_time: f64) -> Option<Bullet> {
        // Check if enough time has passed since last shot
        if current_time - self.last_shot < self.shoot_cooldown {
            return None;
        }

        // Calculate distance to target
        let dx = target_position.0 - self.position.0;
        let dy = target_position.1 - self.position.1;
        let distance = (dx * dx + dy * dy).sqrt();
        
        // Only shoot if target is in range
        if distance <= self.range as f64 {
            self.last_shot = current_time;
            Some(Bullet::new(self.position, target_position, 10.0, self.damage))
        } else {
            None
        }
    }
}
