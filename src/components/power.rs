#[derive(Debug, PartialEq)]
pub struct Powerable {
    level: u16,
    level_base: u16,
}

impl Powerable {
    pub fn new(level: u16, level_base: u16) -> Powerable {
        Powerable {level: level, level_base: level_base}
    }

    pub fn reset(&mut self) {
        self.level = self.level_base;
    }

    pub fn down(&mut self) -> bool {
        if self.level >= 3 {
            self.level -= 3;
            return true;
        }
        else {
            return false;
        }
    }

    pub fn up(&mut self) {
        self.level += 3;
    }

    pub fn is_powered(&self) -> bool {
        self.level > 0
    }

    pub fn get_power_pct(&self) -> f32 {
        (self.level as f32) / (self.level_base as f32)
    }
}