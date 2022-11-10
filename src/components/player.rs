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

#[derive(Debug, PartialEq)]
pub struct Player {
    pub id: usize,
    state: PlayerState,
    pub system_adjust_cooldown: Cooldown,
    aim_control_state: AimControlState,
    pub aim_mode_cooldown: Cooldown,
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

impl Player {
    pub fn new(id: usize, system_adjust_cooldown_reset: f32, aim_mode_cooldown_reset: f32) -> Player {
        Player{ 
            id: id,
            state: PlayerState::Active,
            system_adjust_cooldown: Cooldown::new(0.0, system_adjust_cooldown_reset),
            aim_control_state: AimControlState::Locked,
            aim_mode_cooldown: Cooldown::new(0.0, aim_mode_cooldown_reset),
        }
    }

    pub fn player_state_in_game(&self) -> bool {
        if self.state == PlayerState::Active || self.state == PlayerState::InRespawn {
            return true;
        }
        else {
            return false;
        }
    }

    pub fn aim_control_weapon_active(&self) -> bool {
        if self.aim_control_state == AimControlState::Weapon || self.aim_control_state == AimControlState::Locked {
            return true;
        }
        else {
            return false;
        }
    }

    pub fn aim_control_shield_active(&self) -> bool {
        if self.aim_control_state == AimControlState::Shield || self.aim_control_state == AimControlState::Locked {
            return true;
        }
        else {
            return false;
        }
    }

    pub fn aim_control_set_state(&mut self, state: AimControlState) {
        self.aim_control_state = state;
    }
}