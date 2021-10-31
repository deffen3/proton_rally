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
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}