pub struct Tower {
    damage: u32,
    range: u32,
}

impl Tower {
    pub fn new(damage: u32, range: u32) -> Self {
        Tower { damage, range }
    }

    pub fn get_damage(&self) -> u32 {
        self.damage
    }

    pub fn get_range(&self) -> u32 {
        self.range
    }
}
