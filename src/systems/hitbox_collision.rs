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
        let dt = time.delta_seconds();

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

            let (movable_collider_pos,
                movable_collider_shape) = get_movable_shape_pos(transform, hitbox);

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
                let collision_avoid_id_check1 = match movable2.prevent_collision_id {
                    Some(prevent_id_2) => {entity.id() != prevent_id_2},
                    _ => true,
                };
                let collision_avoid_id_check2 = match movable.prevent_collision_id {
                    Some(prevent_id_1) => {entity2.id() != prevent_id_1},
                    _ => true,
                };

                if (entity.id() != entity2.id()) && collision_avoid_id_check1 && collision_avoid_id_check2 {
                    let (movable2_collider_pos,
                        movable2_collider_shape) = get_movable_shape_pos(transform2, hitbox2);

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
                        (Some(_), Some(_)) => { 
                            //Collision still in progress, repeat collision detected => do nothing
                        }
                        (Some(curr_collision), None) => { 
                            //New collision => calculate reaction
                            let contact_pt = curr_collision.world2;
                            self.collision_ids.insert( (entity.id(), entity2.id()), contact_pt);
                        }
                        (None, Some(_)) => { 
                            // Previous collision still exists, need to "clear" the past collision
                            self.collision_ids.remove( &(entity.id(), entity2.id()) );
                        }                  
                    }
                }
            }
        }

        // Find collision contact pts, but separated out by each entity id
        let mut movable_collisions: HashMap<u32, Vec<Point<f32, U2>>> = HashMap::new();

        for ((e1_id, e2_id), contact_pt) in self.collision_ids.iter() {
            {
                let movable1_contact_pts = movable_collisions.get_mut(&e1_id);

                match movable1_contact_pts {
                    None => {
                        movable_collisions.insert(*e1_id, vec!(*contact_pt));
                    },
                    Some(contact_pts) => {
                        contact_pts.push(*contact_pt);
                    }
                }
            }

            {
                let movable2_contact_pts = movable_collisions.get_mut(&e2_id);
                match movable2_contact_pts {
                    None => {
                        movable_collisions.insert(*e2_id, vec!(*contact_pt));
                    },
                    Some(contact_pts) => {
                        contact_pts.push(*contact_pt);
                    }
                }
            }
        }

        // Resolve collisions
        for (entity, mut movable, _mass, _hitbox, transform) in (
            &entities,
            &mut movables,
            &masses,
            &hitboxes,
            &mut transforms,
        )
            .join()
        {
            if let Some(contact_pts) = movable_collisions.get(&entity.id()) {

                for contact_pt in contact_pts.iter() {
                    let movable_x = transform.translation().x;
                    let movable_y = transform.translation().y;

                    match movable.collision_type {
                        CollisionType::Bounce => {
                            // let impulse = COLLISION_LOSS * (2.0 * movable_weight)
                            //     / (movable_weight + other_movable_weight);

                            let impulse = 10.0;

                            movable.dx = movable.dx - impulse * (contact_pt.x - movable_x);
                            movable.dy = movable.dy - impulse * (contact_pt.y - movable_y);

                            transform.set_translation_x(movable_x + movable.dx * dt);
                            transform.set_translation_y(movable_y + movable.dy * dt);
                        },
                        CollisionType::_Stick => {
                            movable.dx = 0.0;
                            movable.dy = 0.0;
                        },
                        _ => {}
                    }
                }
            }
        }
    }
}