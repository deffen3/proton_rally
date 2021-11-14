use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PlayerState {
    Active,
    //InActive,
    InRespawn,
}

pub struct Player {
    pub id: usize,
    pub state: PlayerState,
    pub system_adjust_cooldown_timer: f32,
    pub system_adjust_cooldown_reset: f32,
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}