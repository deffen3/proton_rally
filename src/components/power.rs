#[derive(Debug, PartialEq)]
pub struct Powerable {
    pub power: u16,
    pub power_base: u16,
}

impl Powerable {
    pub fn reset(&mut self) {
        self.power = self.power_base;
    }

    pub fn down(&mut self) -> bool {
        if self.power >= 3 {
            self.power -= 3;
            return true;
        }
        else {
            return false;
        }
    }

    pub fn up(&mut self) {
        self.power += 3;
    }

    pub fn is_powered(&self) -> bool {
        self.power > 0
    }

    pub fn get_power_pct(&self) -> f32 {
        (self.power as f32) / (self.power_base as f32)
    }
}