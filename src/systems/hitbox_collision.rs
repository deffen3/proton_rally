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
use ncollide2d::query::{self, Proximity};

use crate::components::{Movable, get_movable_shape_pos, CollisionType, Mass, Hitbox};

#[derive(SystemDesc)]
pub struct HitboxCollisionDetection {
    pub collision_ids: HashMap<(u32, u32), Point<f32, U2>>,
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
        let _dt = time.delta_seconds();

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

                    let proximity_detected = query::proximity(
                        &movable_collider_pos,
                        &movable_collider_shape,
                        &movable2_collider_pos,
                        &movable2_collider_shape,
                        collision_margin,
                    );

                    let contact_data = match proximity_detected {
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

                    let previous_collision = self.collision_ids.get( &(entity.id(), entity2.id()) );

                    match (contact_data, previous_collision) {
                        (None, None) => {}, // No current collision, no previous collision => do nothing
                        (Some(curr_collision), Some(prev_collision)) => { 
                            //Collision still in progress, repeat collision detected => do nothin
                            
                        }
                        (Some(curr_collision), None) => { 
                            //New collision => calculate reaction
                            let contact_pt = curr_collision.world2;
                            self.collision_ids.insert( (entity.id(), entity2.id()), contact_pt);
    
                            if movable.collision_type == CollisionType::Bounce && movable2.collision_type == CollisionType::Bounce {
                                log::info!("Collision {:?}, {:?}", entity.id(), entity2.id());
                            }
                        }
                        (None, Some(prev_collision)) => { 
                            // Previous collision still exists, need to "clear" the past collision
                            self.collision_ids.remove( &(entity.id(), entity2.id()) );
                        }                  
                    }
                }
            }
        }
    }
}