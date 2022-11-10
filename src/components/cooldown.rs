#[derive(Debug, PartialEq)]
pub struct Cooldown {
    timer: f32,
    reset: f32,
}

impl Cooldown {
    pub fn new(timer: f32, reset: f32) -> Cooldown {
        Cooldown {timer: timer, reset: reset}
    }

    pub fn timer_update(&mut self, dt: &f32) {
        if self.timer > 0.0 {
            self.timer -= dt;
        }
    }

    pub fn timer_active(&self) -> bool {
        if self.timer > 0.0 {
            return false;
        }
        else {
            return true;
        }
    }

    pub fn timer_reset(&mut self) {
        self.timer = self.reset;
    }

    pub fn timer_reset_multiplier(&mut self, mult: f32) {
        self.timer = self.reset * mult;
    }
}