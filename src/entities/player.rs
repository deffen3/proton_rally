use amethyst::{
    core::{transform::Transform, math::Vector3},
    ecs::prelude::{Entity, World},
    prelude::*,
    renderer::{SpriteRender, Transparent},
};

use std::f32::consts::PI;

use crate::components::{Arena, Movable, CollisionType, Mass, Player, PlayerState, Hitbox, HitboxShape};

pub fn intialize_player(
    world: &mut World,
    arena_properties: &Arena,
    sprite_sheet_handle: &Vec<SpriteRender>,
) {
    for (player_id, player_spawn_point) in arena_properties.player_spawn_points.iter().enumerate() {
        let x_scale = 0.5;
        let y_scale = 0.5;

        let mut player_transform = Transform::default();
        player_transform.set_rotation_2d(player_spawn_point.rotation / 180.0 * PI);
        player_transform.set_translation_xyz(player_spawn_point.x, player_spawn_point.y, 0.0);
        player_transform.set_scale(Vector3::new(
            x_scale,
            y_scale,
            0.0,
        ));

        //Create actual Player with Vehicle and Weapon
        world
            .create_entity()
            .with(player_transform)
            .with(sprite_sheet_handle[player_id].clone())
            .with(Transparent)
            .with(Player{id: player_id, state: PlayerState::Active})
            .with(Movable::new(CollisionType::Bounce))
            .with(Mass::new(1.0))
            .with(Hitbox::new(16.0 * x_scale, 16.0 * y_scale, HitboxShape::Circle))
            .build();
    }
}