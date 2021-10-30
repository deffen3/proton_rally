use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Arena {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
}

impl Component for Arena {
    type Storage = DenseVecStorage<Self>;
}

impl Arena {
    pub fn new(x: f32, y: f32, angle: f32) -> Arena {
        Arena {
            x,
            y,
            angle,
        }
    }
}