use amethyst::ecs::prelude::{Component, DenseVecStorage};

use crate::components::{Cooldown};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PlayerState {
    Active,
    //InActive,
    InRespawn,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AimControlState {
    Weapon,
    Shield,
    Locked,
}

pub struct Player {
    pub id: usize,
    pub state: PlayerState,
    pub system_adjust_cooldown: Cooldown,
    pub aim_control_state: AimControlState,
    pub aim_mode_cooldown: Cooldown,
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}