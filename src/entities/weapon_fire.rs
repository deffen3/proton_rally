use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Entities, Entity, LazyUpdate, ReadExpect},
    utils::removal::Removal,
};

use std::f32::consts::PI;

use crate::components::{CollisionType, Movable, Weapon, WeaponFire, Hitbox, Mass, Powerable};
use crate::resources::WeaponFireResource;


pub fn fire_weapon(
    entities: &Entities,
    entity_id: u32,
    player_id: usize,
    player_transform: &Transform,
    weapon: &Weapon,
    weapon_fire_resource: &ReadExpect<WeaponFireResource>,
    lazy_update: &ReadExpect<LazyUpdate>,
) {
    let weapon_fire_entity: Entity = entities.create();

    let weapon_fire = WeaponFire{};

    let (local_transform, weapon_fire_movable) = {
        let mut local_transform = Transform::default();
        local_transform.set_translation_x(player_transform.translation().x);
        local_transform.set_translation_y(player_transform.translation().y);

        local_transform.set_rotation_2d(weapon.angle - PI);

        let weapon_fire_movable = Movable{
            dx: weapon.shot_speed * -weapon.angle.sin(),
            dy: weapon.shot_speed * weapon.angle.cos(),
            power: Powerable {power: 1, power_base: 1},
            max_accel_force: 0.0,
            collision_type: CollisionType::Bounce{bounces: Some(2), sticks: false},
            prevent_collision_id: Some(entity_id),
        };

        (local_transform, weapon_fire_movable)
    };

    let weapon_fire_hitbox = Hitbox{
        height: 2.0,
        width: 2.0,
        shape: crate::components::HitboxShape::Circle,
    };

    let weapon_fire_mass = Mass{mass: 0.01};

    let weapon_sprite = match player_id {
        0 => weapon_fire_resource.player_1_weapon_fire.clone(),
        1 => weapon_fire_resource.player_2_weapon_fire.clone(),
        2 => weapon_fire_resource.player_3_weapon_fire.clone(),
        3 => weapon_fire_resource.player_4_weapon_fire.clone(),
        _ => weapon_fire_resource.player_1_weapon_fire.clone(),
    };

    lazy_update.insert(weapon_fire_entity, weapon_fire);
    lazy_update.insert(weapon_fire_entity, weapon_fire_movable);
    lazy_update.insert(weapon_fire_entity, weapon_fire_hitbox);
    lazy_update.insert(weapon_fire_entity, weapon_fire_mass);

    lazy_update.insert(weapon_fire_entity, weapon_sprite);
    lazy_update.insert(weapon_fire_entity, local_transform);

    lazy_update.insert(weapon_fire_entity, Removal::new(0 as u32));
}
