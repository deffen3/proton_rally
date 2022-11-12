use amethyst::{
    core::{Time, Transform},
    derive::SystemDesc,
    ecs::{
        Join, Read, System, SystemData, World,
        WriteStorage, ReadStorage, Entities,
    },
};

use std::collections::HashMap;

extern crate nalgebra as na;
use na::{Point, U2};
use na::{Isometry2, Point2, Vector2};
use ncollide2d::query::{self, Proximity, Ray, RayCast};

use crate::components::{Movable, get_movable_shape_pos, CollisionType, Mass, Hitbox};


pub const PRE_IMPACT_DT_STEPS: f32 = 1.1;
pub const TOI_SPEED_TRIGGER: f32 = 200.0;

#[derive(SystemDesc, Default)]
pub struct HitboxCollisionDetection {
}

impl<'s> System<'s> for HitboxCollisionDetection {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Hitbox>,
        WriteStorage<'s, Movable>,
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
            masses,
            mut transforms,
            time,
        ): Self::SystemData,
    ) {
        let dt = time.delta_seconds();

        // Keep track of current entity IDs involved, 
        // as any ID that is not current needs to be removed from collision_ids
        let mut current_ids: Vec<u32> = vec![];

        // For movable, mass, hitboxes
        for (entity, _movable, _mass, _hitbox, _transform) in (
            &entities,
            &movables,
            &masses,
            &hitboxes,
            &transforms,
        )
            .join()
        {
            // For each other movable, mass, hitbox
            for (entity2, movable2, mass2, hitbox2, transform2) in (
                &entities,
                &movables,
                &masses,
                &hitboxes,
                &transforms,
            )
                .join()
            {
                if entity.id() != entity2.id() {

                }
            }
        }
    }
}