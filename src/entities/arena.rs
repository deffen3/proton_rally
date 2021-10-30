use amethyst::{
    core::{transform::Transform, math::Vector3},
    ecs::prelude::{Entity, World},
    prelude::*,
    renderer::{SpriteRender},
};

use std::f32::consts::PI;

use crate::components::{Arena, Hitbox, HitboxShape};

pub fn intialize_arena(
    world: &mut World,
    sprite_sheet_handle: &Vec<SpriteRender>,
) -> Entity {

    let x = 500.0;
    let y = 400.0;
    let angle = 0.0;
    let scale = 5.0;
    let width = 10.0 * scale;
    let height = 50.0 * scale;

    let mut arena_element_transform = Transform::default();
    arena_element_transform.set_rotation_2d(angle);
    arena_element_transform.set_translation_xyz(x, y, 0.0);
    arena_element_transform.set_scale(Vector3::new(
        1.0 * scale,
        1.0 * scale,
        0.0,
    ));

    world
        .create_entity()
        .with(arena_element_transform)
        .with(sprite_sheet_handle[1].clone())
        .with(Arena::new(x, y, angle))
        .with(Hitbox::new(width, height, HitboxShape::Rectangle))
        .build();



    let x = 500.0;
    let y = 400.0;
    let angle = PI/2.0;
    let scale = 5.0;
    let width = 50.0 * scale; //rotated
    let height = 10.0 * scale; //rotated

    let mut arena_element_transform = Transform::default();
    arena_element_transform.set_rotation_2d(angle);
    arena_element_transform.set_translation_xyz(x, y, 0.0);
    arena_element_transform.set_scale(Vector3::new(
        1.0 * scale,
        1.0 * scale,
        0.0,
    ));

    world
        .create_entity()
        .with(arena_element_transform)
        .with(sprite_sheet_handle[1].clone())
        .with(Arena::new(x, y, angle))
        .with(Hitbox::new(width, height, HitboxShape::Rectangle))
        .build()
}