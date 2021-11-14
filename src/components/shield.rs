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