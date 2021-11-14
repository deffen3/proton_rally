use amethyst::ecs::prelude::{Component, DenseVecStorage};


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Weapon {
    pub cooldown_timer: f32,
    pub cooldown_reset: f32,
    pub power: u16,
    pub power_base: u16,
    pub shot_speed: f32,
    pub damage: f32,
    pub angle: f32, //needs to be synchronized with child entity's weapon angle
}

impl Component for Weapon {
    type Storage = DenseVecStorage<Self>;
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WeaponAimChild {
    pub id: usize,
    pub angle: f32, //needs to be synchronized with parent entity's weapon angle
}

impl Component for WeaponAimChild {
    type Storage = DenseVecStorage<Self>;
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WeaponFire {

}

impl Component for WeaponFire {
    type Storage = DenseVecStorage<Self>;
}