#[derive(Debug, PartialEq)]
pub struct Powerable {
    level: u8,
    level_base: u8,
    level_increment: u8
}

impl Powerable {
    pub fn new(level: u8, level_base: u8) -> Powerable {
        Powerable {level: level, level_base: level_base, level_increment: level_base/3}
    }

    pub fn is_powered(&self) -> bool {
        self.level > 0
    }

    pub fn get_power_pct(&self) -> f32 {
        (self.level as f32) / (self.level_base as f32)
    }

    pub fn reset(&mut self) {
        self.level = self.level_base;
    }

    pub fn down(&mut self) -> u8 {
        let power_freed: u8;

        if self.level >= self.level_increment { // dec by max (=increment)
            power_freed = self.level_increment;
            self.level -= self.level_increment;
        }
        else if self.level > 0 { // dec as much as possible (<increment)
            power_freed = self.level;
            self.level = 0;
        }
        else { // can't dec, already at 0
            power_freed = 0;
        }

        return power_freed;
    }

    pub fn up(&mut self, free_power: u8) {
        // increase by amount that all other systems were decreased
        self.level += free_power;
    }

}