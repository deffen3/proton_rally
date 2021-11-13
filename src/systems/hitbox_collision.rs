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
    //track existing collisions, key: entity IDs, values: contact point, masses, velocities
    pub collision_ids: HashMap<(u32, u32), (Point<f32, U2>, f32, f32, f32, f32)>, 
    //track future collisions, key: entity IDs, values: time-of-impact
    pub future_collision_ids_toi: HashMap<(u32, u32), f32>, 
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
            if !current_ids.contains(&entity.id()) {
                current_ids.push(entity.id());
            }
        }

        let mut old_ids_marked_for_removal: Vec<(u32, u32)> = vec![];

        for ((e1_id, e2_id), _) in self.collision_ids.iter() {
            if !current_ids.contains(&e1_id) || !current_ids.contains(&e2_id) {
                if !old_ids_marked_for_removal.contains(&(*e1_id, *e2_id)) {
                    old_ids_marked_for_removal.push((*e1_id, *e2_id));
                }
            }
        }

        for (e1_id, e2_id) in old_ids_marked_for_removal.iter() {
            self.collision_ids.remove( &(*e1_id, *e2_id) );
        }


        // Prep future collision toi for next loop
        // Check for a future toi collision
        let mut toi_to_update: Vec<((u32, u32), f32)> = vec![];
        let mut toi_to_remove: Vec<(u32, u32)> = vec![];

        for (key, toi) in self.future_collision_ids_toi.iter() {
            //self.future_collision_ids_toi.insert(*key, *toi - dt);
            toi_to_update.push((*key, *toi));
        }

        for (key, toi) in toi_to_update.iter() {
            self.future_collision_ids_toi.insert(*key, toi-dt);
        }

        // For movable, mass, hitboxes: perform collision detection logic
        for (entity, movable, mass, hitbox, transform) in (
            &entities,
            &movables,
            &masses,
            &hitboxes,
            &transforms,
        )
            .join()
        {
            // Get Current Positions, Velocities, and Angles

            let abs_vel = (movable.dx.powi(2) + movable.dy.powi(2)).sqrt();

            let collision_margin = match abs_vel {
                abs_vel if abs_vel > TOI_SPEED_TRIGGER => abs_vel * dt * PRE_IMPACT_DT_STEPS,
                _ => 0.0
            };

            let (movable_collider_pos,
                movable_collider_shape) = get_movable_shape_pos(transform, hitbox);

            // For all other movable, mass, hitboxes
            for (entity2, movable2, mass2, hitbox2, transform2) in (
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
                    // Check for a future toi collision
                    let future_collision_toi = self.future_collision_ids_toi.get_mut( &(entity.id(), entity2.id()) );

                    // Update Movable Collider Position based on toi
                    let movable_collider_pos_toi = match future_collision_toi {
                        Some(toi) if *toi < 0.0 => { //toi expired, impact
                            log::info!("future collision toi now expired {:?}, {:?}, {:?}", entity.id(), entity2.id(), toi);

                            //self.future_collision_ids_toi.remove( &(entity.id(), entity2.id()) );
                            toi_to_remove.push((entity.id(), entity2.id()));

                            // Rewind x and y position based on velocity and toi
                            let x = transform.translation().x - movable.dx*(*toi);
                            let y = transform.translation().y - movable.dy*(*toi);
                        
                            let rotation = transform.rotation();
                            let (_, _, angle) = rotation.euler_angles();
                        
                            // New updated toi position
                            Isometry2::new(Vector2::new(x, y), angle)
                        },
                        _ => {
                            movable_collider_pos
                        }
                    };


                    let (movable2_collider_pos,
                        movable2_collider_shape) = get_movable_shape_pos(transform2, hitbox2);

                    let proximity_detected = query::proximity(
                        &movable_collider_pos_toi,
                        &movable_collider_shape,
                        &movable2_collider_pos,
                        &movable2_collider_shape,
                        collision_margin,
                    );

                    let contact_data = match proximity_detected {
                        Proximity::Intersecting => {
                            log::info!("Intersecting: {:?}, {:?}", entity.id(), entity2.id());

                            query::contact(
                                &movable_collider_pos_toi,
                                &movable_collider_shape,
                                &movable2_collider_pos,
                                &movable2_collider_shape,
                                0.0,
                            )
                        },
                        Proximity::WithinMargin => {
                            log::info!("WithinMargin: {:?}, {:?}", entity.id(), entity2.id());

                            let fire_ray = Ray::new(
                                Point2::new(transform.translation().x, transform.translation().y),
                                Vector2::new(movable.dx, movable.dy),
                            );

                            // Time of impact
                            let toi = movable2_collider_shape.toi_with_ray(
                                &movable2_collider_pos,
                                &fire_ray,
                                dt * PRE_IMPACT_DT_STEPS,
                                true,
                            );

                            if let Some(toi) = toi {
                                self.future_collision_ids_toi.insert( (entity.id(), entity2.id()), toi);

                                log::info!("future collision toi {:?}, {:?}, {:?}", entity.id(), entity2.id(), toi);
                            }


                            // Look backwards as well
                            let fire_ray = Ray::new(
                                Point2::new(transform.translation().x, transform.translation().y),
                                Vector2::new(-movable.dx, -movable.dy),
                            );

                            // Time of impact
                            let toi = movable2_collider_shape.toi_with_ray(
                                &movable2_collider_pos,
                                &fire_ray,
                                dt * PRE_IMPACT_DT_STEPS,
                                true,
                            );

                            if let Some(toi) = toi {
                                log::info!("future collision -toi {:?}, {:?}, {:?}", entity.id(), entity2.id(), toi);
                            }

                            None
                        }
                        _ => None,
                    };

                    let previous_collision = self.collision_ids.get( &(entity.id(), entity2.id()) );

                    match (contact_data, previous_collision) {
                        (None, None) => {}, // No current collision, no previous collision => do nothing
                        (Some(_), Some(_)) => { 
                            //Collision still in progress, repeat collision detected => do nothing
                            log::info!("movable collision arb none {:?}, {:?}", entity.id(), entity2.id());
                        }
                        (Some(curr_collision), None) => { 
                            //New collision => calculate reaction
                            let contact_pt = curr_collision.world2;
                            self.collision_ids.insert(
                                 (entity.id(), entity2.id()),
                                 (contact_pt, mass.mass, mass2.mass, movable.dx - movable2.dx, movable.dy - movable2.dy));
                            log::info!("movable collision arb new {:?}, {:?}", entity.id(), entity2.id());
                        }
                        (None, Some(_)) => { 
                            // Previous collision still exists, need to "clear" the past collision
                            self.collision_ids.remove( &(entity.id(), entity2.id()) );
                            log::info!("movable collision arb rem prev {:?}, {:?}", entity.id(), entity2.id());
                        }                  
                    }
                }
            }
        }       


        // Find collision contact pts, but separated out by each entity id
        let mut movable_collisions: HashMap<u32, Vec<(Point<f32, U2>, f32, f32, f32, f32)>> = HashMap::new();

        for ((e1_id, e2_id), (contact_pt, m1, m2, dx, dy)) in self.collision_ids.iter() {
            log::info!("movable collision {:?}, {:?}", e1_id, e2_id);

            {
                let movable1_contact_pts = movable_collisions.get_mut(&e1_id);

                match movable1_contact_pts {
                    None => {
                        movable_collisions.insert(*e1_id, vec!((*contact_pt, *m1, *m2, *dx, *dy)));
                    },
                    Some(values) => {
                        values.push((*contact_pt, *m1, *m2, *dx, *dy));
                    }
                }
            }

            {
                let movable2_contact_pts = movable_collisions.get_mut(&e2_id);
                match movable2_contact_pts {
                    None => {
                        movable_collisions.insert(*e2_id, vec!((*contact_pt, *m1, *m2, *dx, *dy)));
                    },
                    Some(values) => {
                        values.push((*contact_pt, *m2, *m1, -*dx, -*dy));
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
            if let Some(contact_pt_and_masses) = movable_collisions.get(&entity.id()) {
                log::info!("movable collision res {:?}",entity.id());

                for (contact_pt, m1, m2, _dx, _dy) in contact_pt_and_masses.iter() {
                    let movable_x = transform.translation().x;
                    let movable_y = transform.translation().y;

                    let contact_diff_x = contact_pt.x - movable_x;
                    let contact_diff_y = contact_pt.y - movable_y;

                    let contact_diff_angle = contact_diff_y.atan2(contact_diff_x);
                    let contact_diff_pct_x = contact_diff_angle.cos();
                    let contact_diff_pct_y = contact_diff_angle.sin();

                    let _mass_ratio = 1.0 - (m1 / (m1/m2));

                    match movable.collision_type {
                        CollisionType::Bounce {bounces, sticks} => {
                            match (bounces, sticks) {
                                (Some(b), _) if b > 0 => { //bounce
                                    movable.collision_type = CollisionType::Bounce {bounces: Some(b-1), sticks};
                            
                                    //movable.dx = movable.dx + dx*(mass_ratio * contact_diff_pct_x);
                                    //movable.dy = movable.dy + dy*(mass_ratio * contact_diff_pct_y);
                                    movable.dx = movable.dx - 10.0 * contact_diff_pct_x;
                                    movable.dy = movable.dy - 10.0 * contact_diff_pct_y;

                                    transform.set_translation_x(movable_x + movable.dx * dt);
                                    transform.set_translation_y(movable_y + movable.dy * dt);
                                },
                                (Some(b), false) if b == 0 => { //all bounces used up, now dissappears
                                    movable.dx = 0.0;
                                    movable.dy = 0.0;
    
                                    log::info!("delete {:?}",entity.id());
                                    let _ = entities.delete(entity);
                                },
                                (Some(b), true) if b == 0 => { //all bounces used up, now sticks
                                    movable.dx = 0.0;
                                    movable.dy = 0.0;
                                },
                                (None, _) => { //infinite bounces

                                    //movable.dx = movable.dx + dx*(mass_ratio * contact_diff_pct_x);
                                    //movable.dy = movable.dy + dy*(mass_ratio * contact_diff_pct_y);
                                    movable.dx = movable.dx - 10.0 * contact_diff_pct_x;
                                    movable.dy = movable.dy - 10.0 * contact_diff_pct_y;

                                    transform.set_translation_x(movable_x + movable.dx * dt);
                                    transform.set_translation_y(movable_y + movable.dy * dt);
                                },
                                _ => {}
                            }
                        },
                        _ => {}
                    }
                }
            }
        }

        // Clean-up future collision toi for next loop
        for key in toi_to_remove.iter() {
            self.future_collision_ids_toi.remove(key);
        }
    }
}