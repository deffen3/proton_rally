use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Movable {
    pub width: f32,
    pub height: f32,
    pub dx: f32,
    pub dy: f32,
}

impl Component for Movable {
    type Storage = DenseVecStorage<Self>;
}

impl Movable {
    pub fn new(width: f32, height: f32) -> Movable {
        Movable {
            width,
            height,
            dx: 0.0,
            dy: 0.0,
        }
    }
}