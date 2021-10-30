use amethyst::ecs::prelude::{Component, DenseVecStorage};

use serde::Deserialize;

// Just used to store the current orthographic projection of the camera
#[derive(Copy, Clone, Debug, PartialEq, Deserialize)]
pub struct CameraOrtho {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
}

impl Component for CameraOrtho {
    type Storage = DenseVecStorage<Self>;
}