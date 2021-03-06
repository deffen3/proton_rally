use amethyst::ecs::prelude::{Component, DenseVecStorage};

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
    pub system_adjust_cooldown_timer: f32,
    pub system_adjust_cooldown_reset: f32,
    pub aim_control_state: AimControlState,
    pub aim_mode_cooldown_timer: f32,
    pub aim_mode_cooldown_reset: f32,
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}