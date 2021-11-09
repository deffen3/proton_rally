use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Entities, Entity, LazyUpdate, ReadExpect},
    utils::removal::Removal,
};

use std::f32::consts::PI;

use crate::components::{CollisionType, Movable, Weapon, WeaponFire};
use crate::resources::WeaponFireResource;


pub fn fire_weapon(
    entities: &Entities,
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
            collision_type: CollisionType::Bounce,
        };

        (local_transform, weapon_fire_movable)
    };

    let weapon_sprite = weapon_fire_resource.player_1_weapon_fire.clone();

    lazy_update.insert(weapon_fire_entity, weapon_fire);
    lazy_update.insert(weapon_fire_entity, weapon_fire_movable);

    lazy_update.insert(weapon_fire_entity, weapon_sprite);
    lazy_update.insert(weapon_fire_entity, local_transform);

    lazy_update.insert(weapon_fire_entity, Removal::new(0 as u32));
}
