use amethyst::{
    core::{Time, Transform},
    derive::SystemDesc,
    ecs::{
        Join, Read, System, SystemData, World,
        WriteStorage, ReadStorage, Entities,
    },
};

extern crate nalgebra as na;
use log::debug;
use ncollide2d::{
    na::{Isometry2, Point2, Vector2},
    query::{self, Proximity, Ray, RayCast}, 
    shape::{Shape, Ball, CompositeShape},
};

use crate::components::{Movable, CollisionType, Mass, Hitbox};


pub const PRE_IMPACT_DT_STEPS: f32 = 1.1;
pub const TOI_SPEED_TRIGGER: f32 = 200.0;

#[derive(SystemDesc, Default)]
pub struct HitboxCollisionDetection {}

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
        for (entity1, movable1, mass1, hitbox1, transform1) in (
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
                if entity1.id() != entity2.id() {
                    match (hitbox1.collider.as_shape::<Ball<f32>>(), hitbox2.collider.as_shape::<Ball<f32>>()) {
                        (Some(g1), Some(g2)) => {
                            let proximity_res = query::proximity(
                                &Isometry2::new(Vector2::new(
                                    transform1.translation().x, transform1.translation().y), 0.0),
                                g1,
                                &Isometry2::new(Vector2::new(
                                    transform2.translation().x, transform2.translation().y), 0.0),
                                g2,
                                0.001
                            );
                            println!("Proximity {:?}", proximity_res);
                        }
                        _ => {
                        }
                    }
                }
            }
        }
    }
}