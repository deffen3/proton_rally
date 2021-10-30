use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum HitboxShape {
    Rectangle,
    Circle
}

pub struct Hitbox {
    pub width: f32,
    pub height: f32,
    pub shape: HitboxShape,
}

impl Component for Hitbox {
    type Storage = DenseVecStorage<Self>;
}

impl Hitbox {
    pub fn new(width: f32, height: f32, shape: HitboxShape) -> Hitbox {
        Hitbox {
            width,
            height,
            shape,
        }
    }
}