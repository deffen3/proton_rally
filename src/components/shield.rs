use amethyst::ecs::prelude::{Component, DenseVecStorage};


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Shield {
    pub cooldown_timer: f32,
    pub cooldown_reset: f32,
    pub power: u16,
    pub power_base: u16,
    pub angle: f32, //needs to be synchronized with child entity's weapon angle
}

impl Component for Shield {
    type Storage = DenseVecStorage<Self>;
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ShieldAimChild {
    pub id: usize,
    pub angle: f32, //needs to be synchronized with parent entity's weapon angle
}

impl Component for ShieldAimChild {
    type Storage = DenseVecStorage<Self>;
}


pub fn calc_shield_block_angle(
    player_x: f32,
    player_y: f32,
    contact_x: f32,
    contact_y: f32,
    shield_angle: f32,
    shield_coverage_deg: f32,
) -> bool {
    true
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_calc_shield_block_angle() {
        assert!(calc_shield_block_angle(
            0.0, 0.0,
            1.0, 0.0,
            0.0, 30.0,
        ) == true);
    }
}
