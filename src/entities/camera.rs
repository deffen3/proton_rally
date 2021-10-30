use amethyst::{
    core::transform::{Parent, Transform},
    ecs::prelude::{Entity, World},
    prelude::*,
    renderer::{Camera},
};

use crate::components::Arena;

pub fn initialize_camera(world: &mut World, arena_properties: &Arena) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(
        arena_properties.width * 0.5,
        arena_properties.height * 0.5,
        1.0,
    );

    world
        .create_entity()
        // .with(Camera::standard_2d(arena_properties.width, arena_properties.height))
        .with(Camera::orthographic(
            -arena_properties.width/2.0,
            arena_properties.width/2.0,
            -arena_properties.height/2.0,
            arena_properties.height/2.0,
            0.0,
            5.0,
        ))
        .with(transform)
        .build();
}