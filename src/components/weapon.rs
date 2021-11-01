use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Weapon {
    pub id: usize,
    pub angle: f32,
}

impl Component for Weapon {
    type Storage = DenseVecStorage<Self>;
}