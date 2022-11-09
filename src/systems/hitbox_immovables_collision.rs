use amethyst::{
    core::{Time, Transform},
    derive::SystemDesc,
    ecs::{
        Entities, Join, Read, System, SystemData, World,
        WriteStorage, ReadStorage,
    },
};

extern crate nalgebra as na;
use na::{Isometry2, Vector2};
use ncollide2d::query::{self, Proximity};
use ncollide2d::shape::{Ball, Cuboid};

use crate::components::{
    ArenaElement, Movable, get_movable_shape_pos, 
    CollisionType, calc_bounce_angle, Mass, Hitbox, HitboxShape};

#[derive(SystemDesc, Default)]
pub struct HitboxImmovableCollisionDetection {
}

impl<'s> System<'s> for HitboxImmovableCollisionDetection {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Hitbox>,
        WriteStorage<'s, Movable>,
        ReadStorage<'s, ArenaElement>,
        ReadStorage<'s, Mass>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn setup(&mut self, _world: &mut World) {
    }

    fn run(
        &mut self,
        (
            entities,
            hitboxes,
            mut movables,
            arena_elements,
            masses,
            mut transforms,
            time,
        ): Self::SystemData,
    ) {
        let dt = time.delta_seconds();

        // For movable, mass, hitboxes
        for (entity, movable, mass, hitbox, transform) in (
            &entities,
            &mut movables,
            &masses,
            &hitboxes,
            &mut transforms,
        )
            .join()
        {
            // For non-movable arena hitboxes
            for (arena_hitbox, arena_element) in (
                &hitboxes,
                &arena_elements,
            )
                .join()
            {
                
            }
        }
    }
}