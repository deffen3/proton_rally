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
            let collision_margin = 5.0;

            // Get Current Positions, Velocities, and Angles
            let (movable_collider_pos, movable_collider_shape) = get_movable_shape_pos(transform, hitbox);

            // For non-movable arena hitboxes
            for (arena_hitbox, arena_element) in (
                &hitboxes,
                &arena_elements,
            )
                .join()
            {
                let immovable_x = arena_element.x;
                let immovable_y = arena_element.y;

                let contact_data;

                if arena_hitbox.shape == HitboxShape::Rectangle {
                    let immovable_collider_shape = Cuboid::new(Vector2::new(
                        arena_hitbox.width / 2.0,
                        arena_hitbox.height / 2.0,
                    ));
                    let immovable_collider_pos = Isometry2::new(Vector2::new(immovable_x, immovable_y), 0.0);

                    let collision = query::proximity(
                        &movable_collider_pos,
                        &movable_collider_shape,
                        &immovable_collider_pos,
                        &immovable_collider_shape,
                        collision_margin,
                    );

                    contact_data = match collision {
                        Proximity::Intersecting => {
                            query::contact(
                                &movable_collider_pos,
                                &movable_collider_shape,
                                &immovable_collider_pos,
                                &immovable_collider_shape,
                                0.0,
                            )
                        },
                        _ => None,
                    };
                }
                else if arena_hitbox.shape == HitboxShape::Circle {
                    let immovable_collider_shape = Ball::new(arena_hitbox.width / 2.0);
                    let immovable_collider_pos = Isometry2::new(Vector2::new(immovable_x, immovable_y), 0.0);

                    let collision = query::proximity(
                        &movable_collider_pos,
                        &movable_collider_shape,
                        &immovable_collider_pos,
                        &immovable_collider_shape,
                        collision_margin,
                    );

                    contact_data = match collision {
                        Proximity::Intersecting => {
                            query::contact(
                                &movable_collider_pos,
                                &movable_collider_shape,
                                &immovable_collider_pos,
                                &immovable_collider_shape,
                                0.0,
                            )
                        },
                        _ => None,
                    };
                }
                else {
                    contact_data = None;
                }

                match contact_data {
                    None => (),
                    Some(cd) => {
                        let contact_pt = cd.world2;

                        log::info!("immovable collision {:?}",entity.id());

                        match movable.collision_type {
                            CollisionType::Bounce {bounces, sticks} => {
                                let (new_dx, new_dy) = calc_bounce_angle(
                                    immovable_x - contact_pt.x,
                                    immovable_y - contact_pt.y,
                                    arena_hitbox.width / 2.0,
                                    arena_hitbox.height / 2.0,
                                    arena_hitbox.shape,
                                    movable.dx.clone(),
                                    movable.dy.clone(),
                                );

                                match (bounces, sticks) {
                                    (Some(b), _) if b > 0 => { //bounce
                                        movable.collision_type = CollisionType::Bounce {bounces: Some(b-1), sticks};
                                    
                                        movable.dx = new_dx * (1.0 as f32).exp().powf(-0.5*mass.mass);
                                        movable.dy = new_dy * (1.0 as f32).exp().powf(-0.5*mass.mass);

                                        let movable_x = transform.translation().x;
                                        let movable_y = transform.translation().y;
                                        transform.set_translation_x(movable_x - (contact_pt.x - movable_x) / 10. + movable.dx * dt);
                                        transform.set_translation_y(movable_y - (contact_pt.y - movable_y) / 10. + movable.dy * dt);
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
                                        movable.dx = new_dx * (1.0 as f32).exp().powf(-0.5*mass.mass);
                                        movable.dy = new_dy * (1.0 as f32).exp().powf(-0.5*mass.mass);

                                        let movable_x = transform.translation().x;
                                        let movable_y = transform.translation().y;
                                        transform.set_translation_x(movable_x - (contact_pt.x - movable_x) / 10. + movable.dx * dt);
                                        transform.set_translation_y(movable_y - (contact_pt.y - movable_y) / 10. + movable.dy * dt);
                                    },
                                    _ => {}
                                }
                            },
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}