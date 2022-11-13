use amethyst::ecs::prelude::{Component, DenseVecStorage};

use serde::Deserialize;

use std::collections::HashMap;

use super::HitboxProperties;


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
    pub rotation: i16, //degrees
}

impl Component for ArenaElement {
    type Storage = DenseVecStorage<Self>;
}

// The properties of ArenaElementHitbox should never be accessed after arena initialization from .ron file,
// instead the components formed from these properties should be accessed
#[derive(Copy, Clone, Debug, PartialEq, Deserialize)]
pub struct ArenaElementHitbox {
    pub element: ArenaElement,
    pub hitbox: HitboxProperties,
}

#[derive(Copy, Clone, Debug, PartialEq, Deserialize)]
pub struct PlayerSpawnPoint {
    pub x: f32,
    pub y: f32,
    pub rotation: f32, //degrees, only supports multiples of 90deg angles due to rectangular hitbox logic
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