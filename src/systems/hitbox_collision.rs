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
use ncollide2d::query::{self, Proximity};

use crate::components::{Movable, get_movable_shape_pos, CollisionType, Mass, Hitbox};

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

        let mut collision_ids = HashMap::new();

        // For movable, mass, hitboxes
        for (entity, movable, _mass, hitbox, transform) in (
            &entities,
            &movables,
            &masses,
            &hitboxes,
            &transforms,
        )
            .join()
        {
            let collision_margin = 5.0;

            // Get Current Positions, Velocities, and Angles

            let (movable_collider_pos, movable_collider_shape) = get_movable_shape_pos(transform, hitbox);

            // For all other movable, mass, hitboxes
            for (entity2, movable2, _mass2, hitbox2, transform2) in (
                &entities,
                &movables,
                &masses,
                &hitboxes,
                &transforms,
            )
                .join()
            {
                if entity.id() != entity2.id() {
                    let (movable2_collider_pos, movable2_collider_shape) = get_movable_shape_pos(transform2, hitbox2);

                    let collision = query::proximity(
                        &movable_collider_pos,
                        &movable_collider_shape,
                        &movable2_collider_pos,
                        &movable2_collider_shape,
                        collision_margin,
                    );

                    let contact_data = match collision {
                        Proximity::Intersecting => {
                            query::contact(
                                &movable_collider_pos,
                                &movable_collider_shape,
                                &movable2_collider_pos,
                                &movable2_collider_shape,
                                0.0,
                            )
                        },
                        _ => None,
                    };

                    match contact_data {
                        None => (),
                        Some(cd) => {
                            let contact_pt = cd.world2;
                            collision_ids.insert(entity.id(), (entity2.id(), contact_pt));
    
                            if movable.collision_type == CollisionType::Bounce && movable2.collision_type == CollisionType::Bounce {
                                log::info!("Collision {:?}, {:?}", entity.id(), entity2.id());
                            }
                        }
                    }
                }
            }
        }
    }
}