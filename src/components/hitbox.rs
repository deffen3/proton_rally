use amethyst::ecs::prelude::{Component, DenseVecStorage};

use serde::Deserialize;

use ncollide2d::na::{Vector2};
use ncollide2d::query::{self, Proximity};
use ncollide2d::shape::{Ball, Cuboid, ShapeHandle};


#[derive(Copy, Clone, Debug, PartialEq, Deserialize)]
pub enum HitboxShape {
    Rectangle,
    Circle
}

#[derive(Copy, Clone, Debug, PartialEq, Deserialize)]
pub struct HitboxProperties {
    pub width: f32,
    pub height: f32,
    pub shape: HitboxShape,
}

impl Component for HitboxProperties {
    type Storage = DenseVecStorage<Self>;
}

pub struct Hitbox {
    pub props: HitboxProperties,
    pub collider: Box<ShapeHandle<f32>>
}

impl Component for Hitbox {
    type Storage = DenseVecStorage<Self>;
}

impl Hitbox {
    pub fn new(width: f32, height: f32, shape: HitboxShape) -> Hitbox {
        let collider: ShapeHandle<f32> = match shape {
            HitboxShape::Circle => {
                ShapeHandle::new(Ball::new(width))
            },
            HitboxShape::Rectangle => {
                ShapeHandle::new(Cuboid::new(Vector2::new(width/2.0, height/2.0)))
            }
        };

        Hitbox {
            props: HitboxProperties {width, height, shape},
            collider: Box::new(collider)}
    }
}