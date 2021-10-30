use amethyst::ecs::prelude::{Component, DenseVecStorage};

use serde::Deserialize;

use std::collections::HashMap;

use super::Hitbox;


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Deserialize)]
pub enum ArenaElementKinds {
    Open,
    Wall,
    Zone,
}

#[derive(Copy, Clone, Debug, PartialEq, Deserialize)]
pub struct ArenaElement {
    pub kind: ArenaElementKinds,
    pub x: f32,
    pub y: f32,
}

impl Component for ArenaElement {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Copy, Clone, Debug, PartialEq, Deserialize)]
pub struct ArenaElementHitbox {
    pub element: ArenaElement,
    pub hitbox: Hitbox,
}

#[derive(Copy, Clone, Debug, PartialEq, Deserialize)]
pub struct PlayerSpawnPoint {
    pub x: f32,
    pub y: f32,
    pub rotation: f32, //degrees
}

#[derive(Copy, Clone, Debug, PartialEq, Deserialize)]
pub struct ArenaFloor {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Default)]
pub struct Arena {
    pub width: f32,
    pub height: f32,
    pub floor: Vec<ArenaFloor>,
    pub player_spawn_points: Vec<PlayerSpawnPoint>,
    pub arena_elements: Vec<ArenaElementHitbox>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Deserialize)]
pub enum ArenaNames {
    OpenEmptyMap,
    StandardCombat,
    StandardKingOfTheHill,
    StandardRace,
    ChaosCombat,
    LargeCombat,
}

pub struct ArenaStoreResource {
    pub properties: HashMap<ArenaNames, Arena>,
}