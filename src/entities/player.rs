use amethyst::{
    core::{transform::Transform, math::Vector3},
    ecs::prelude::{Entity, World},
    prelude::*,
    renderer::{SpriteRender},
};

use crate::components::{Movable, CollisionType, Mass, Player, Hitbox, HitboxShape};

pub fn intialize_player(
    world: &mut World,
    sprite_sheet_handle: &Vec<SpriteRender>,
) -> Entity {

    let mut proton_transform = Transform::default();
    proton_transform.set_rotation_2d(0.0);
    proton_transform.set_translation_xyz(20.0, 20.0, 0.0);
    proton_transform.set_scale(Vector3::new(
        1.0,
        1.0,
        0.0,
    ));

    //Create actual Player with Vehicle and Weapon
    world
        .create_entity()
        .with(proton_transform)
        .with(sprite_sheet_handle[0].clone())
        .with(Player::new(0))
        .with(Movable::new(CollisionType::Bounce))
        .with(Mass::new(1.0))
        .with(Hitbox::new(16.0, 16.0, HitboxShape::Circle))
        .build()
}