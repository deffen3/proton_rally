use amethyst::{
    core::{transform::Transform, math::Vector3, components::Parent},
    ecs::prelude::{World},
    prelude::*,
    renderer::{SpriteRender, Transparent},
};

use std::f32::consts::PI;

use crate::components::{
    Arena, Movable, CollisionType, Mass,
    Player, PlayerState, AimControlState, Hitbox, HitboxShape, Weapon, WeaponAimChild, Shield, ShieldAimChild, Powerable, Cooldown};

pub fn intialize_player(
    world: &mut World,
    arena_properties: &Arena,
    sprite_sheet_handle: &Vec<SpriteRender>,
) {
    for (player_id, player_spawn_point) in arena_properties.player_spawn_points.iter().enumerate() {
        let x_scale = 0.5;
        let y_scale = 0.5;

        let player_rotation = player_spawn_point.rotation / 180.0 * PI;

        //Create player proton body
        let mut player_transform = Transform::default();
        player_transform.set_rotation_2d(player_rotation);
        player_transform.set_translation_xyz(player_spawn_point.x, player_spawn_point.y, 0.0);
        player_transform.set_scale(Vector3::new(x_scale, y_scale, 0.0));

        let power = 9;

        let proton_body = world
            .create_entity()
            .with(player_transform)
            .with(sprite_sheet_handle[player_id].clone())
            .with(Transparent)
            .with(Player{
                id: player_id,
                state: PlayerState::Active,
                system_adjust_cooldown: Cooldown::new(0.0, 0.1),
                aim_control_state: AimControlState::Locked,
                aim_mode_cooldown: Cooldown::new(0.0, 0.1),
            })
            .with(Movable::new(
                9,
                300.0,
                CollisionType::Bounce{bounces:None, sticks:false}))
            .with(Mass::new(1.0))
            .with(Hitbox{
                width: 16.0 * x_scale,
                height: 16.0 * y_scale,
                shape: HitboxShape::Circle})
            .with(Shield{
                cooldown: Cooldown::new(0.0, 0.333),
                power: Powerable::new(power, power),
                angle: player_rotation})
            .with(Weapon{
                cooldown: Cooldown::new(0.0, 0.333),
                power: Powerable::new(power, power),
                shot_speed: 300.0,
                damage: 10.0,
                angle: player_rotation})
            .build();


        //Create player proton shield
        let mut shield_transform = Transform::default();
        shield_transform.set_rotation_2d(0.0);
        shield_transform.set_translation_xyz(0.0, 0.0, 0.0);
        shield_transform.set_scale(Vector3::new(1.0, 1.0, 0.0));

        world
            .create_entity()
            .with(shield_transform)
            .with(sprite_sheet_handle[13].clone())
            .with(Transparent)
            .with(Parent{entity: proton_body})
            .with(ShieldAimChild{id: player_id, angle: player_rotation})
            .build();

        //Create player proton cannon weapon
        let mut cannon_transform = Transform::default();
        cannon_transform.set_rotation_2d(0.0);
        cannon_transform.set_translation_xyz(0.0, 0.0, 0.0);
        cannon_transform.set_scale(Vector3::new(1.0, 1.0, 0.0));

        world
            .create_entity()
            .with(cannon_transform)
            .with(sprite_sheet_handle[6].clone())
            .with(Transparent)
            .with(Parent{entity: proton_body})
            .with(WeaponAimChild{id: player_id, angle: player_rotation})
            .build();
    }
}