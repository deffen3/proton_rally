use amethyst::{
    core::{Time, Transform},
    derive::SystemDesc,
    ecs::{
        Join, Read, System, SystemData, World,
        WriteStorage, ReadStorage,
    },
};

use log::info;

extern crate nalgebra as na;
use na::{Isometry2, Vector2};
use ncollide2d::query::{self, Proximity};
use ncollide2d::shape::{Ball, Cuboid};

use crate::components::{ArenaElement, Movable, CollisionType, calc_bounce_angle, Mass, Hitbox, HitboxShape};

#[derive(SystemDesc, Default)]
pub struct HitboxCollisionDetection {
}

impl<'s> System<'s> for HitboxCollisionDetection {
    type SystemData = (
        ReadStorage<'s, Hitbox>,
        WriteStorage<'s, Movable>,
        ReadStorage<'s, ArenaElement>,
        ReadStorage<'s, Mass>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn setup(&mut self, world: &mut World) {
    }

    fn run(
        &mut self,
        (
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
        for (movable, mass, hitbox, transform) in (
            &mut movables,
            &masses,
            &hitboxes,
            &mut transforms,
        )
            .join()
        {
            let wall_hit_bounce_decel_pct = 0.40;

            // Get Current Positions, Velocities, and Angles

            let movable_x = transform.translation().x;
            let movable_y = transform.translation().y;

            let movable_rotation = transform.rotation();
            let (_, _, movable_angle) = movable_rotation.euler_angles();

            let collision_margin = 5.0;

            let movable_collider_shape =
                Ball::new(hitbox.width / 2.0);
            let movable_collider_pos =
                Isometry2::new(Vector2::new(movable_x, movable_y), movable_angle);

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

                        if movable.collision_type == CollisionType::Bounce {
                            let (new_dx, new_dy) = calc_bounce_angle(
                                immovable_x - contact_pt.x,
                                immovable_y - contact_pt.y,
                                arena_hitbox.width / 2.0,
                                arena_hitbox.height / 2.0,
                                arena_hitbox.shape,
                                movable.dx.clone(),
                                movable.dy.clone(),
                            );

                            movable.dx = new_dx * wall_hit_bounce_decel_pct;
                            movable.dy = new_dy * wall_hit_bounce_decel_pct;

                            transform.set_translation_x(movable_x - (contact_pt.x - movable_x) / 10. + movable.dx * dt);
                            transform.set_translation_y(movable_y - (contact_pt.y - movable_y) / 10. + movable.dy * dt);
                        }
                    }
                }
            }
        }
    }
}