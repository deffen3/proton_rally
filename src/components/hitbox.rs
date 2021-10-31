use amethyst::ecs::prelude::{Component, DenseVecStorage};

use serde::Deserialize;

#[derive(Copy, Clone, Debug, PartialEq, Deserialize)]
pub enum HitboxShape {
    Rectangle,
    Circle
}

#[derive(Copy, Clone, Debug, PartialEq, Deserialize)]
pub struct Hitbox {
    pub width: f32,
    pub height: f32,
    pub shape: HitboxShape,
}

impl Component for Hitbox {
    type Storage = DenseVecStorage<Self>;
}