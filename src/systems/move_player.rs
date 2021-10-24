use amethyst::{
    core::{Time, Transform},
    derive::SystemDesc,
    ecs::{
        Entities, Join, Read, System, SystemData, World,
        WriteStorage, ReadStorage,
    },
    input::{InputHandler, StringBindings},
};

use log::info;

use std::f32::consts::PI;

use crate::components::{Movable, Player};

#[derive(SystemDesc, Default)]
pub struct MovePlayerSystem {
}

impl<'s> System<'s> for MovePlayerSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Player>,
        WriteStorage<'s, Movable>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        Read<'s, InputHandler<StringBindings>>, //<MovementBindingTypes>
    );

    fn setup(&mut self, world: &mut World) {
    }

    fn run(
        &mut self,
        (
            entities,
            players,
            mut movables,
            mut transforms,
            time,
            input,
        ): Self::SystemData,
    ) {
        let dt = time.delta_seconds();

        for (player, mut movable, mut transform) in (
            &players,
            &mut movables,
            &mut transforms,
        )
            .join()
        {
            // let movable_x = transform.translation().x;
            // let movable_y = transform.translation().y;

            // let movable_rotation = transform.rotation();
            // let (_, _, movable_angle) = movable_rotation.euler_angles();

            let vehicle_accel_x = match player.id {
                0 => input.axis_value("p1_accel_x"),
                1 => input.axis_value("p2_accel_x"),
                2 => input.axis_value("p3_accel_x"),
                3 => input.axis_value("p4_accel_x"),
                _ => None,
            }.unwrap_or(0.0);

            let vehicle_accel_y = match player.id {
                0 => input.axis_value("p1_accel_y"),
                1 => input.axis_value("p2_accel_y"),
                2 => input.axis_value("p3_accel_y"),
                3 => input.axis_value("p4_accel_y"),
                _ => None,
            }.unwrap_or(0.0);

            let accel_scalar = 100.0;
            let automatic_decel_scalar = 150.0;
            //let thrust_friction_decel = 0.25;

            // Apply Acceleration
            if vehicle_accel_x.abs() > 0.0 {
                movable.dx += accel_scalar * vehicle_accel_x * dt;
            }
            else if movable.dx.abs() > 0.01 {
                movable.dx -= automatic_decel_scalar * movable.dx.signum() * dt;
            }
            else {
                movable.dx = 0.0;
            }

            if vehicle_accel_y.abs() > 0.0 {
                movable.dy += accel_scalar * vehicle_accel_y * dt;
            }
            else if movable.dy.abs() > 0.01 {
                movable.dy -= automatic_decel_scalar * movable.dy.signum() * dt;
            }
            else {
                movable.dy = 0.0;
            }
            

            // Apply Friction Decel
            // movable.dx -= thrust_friction_decel * movable.dx * dt;
            // movable.dy -= thrust_friction_decel * movable.dy * dt;

            // Transform on vehicle velocity
            transform.prepend_translation_x(movable.dx * dt);
            transform.prepend_translation_y(movable.dy * dt);

            let velocity_angle = movable.dy.atan2(movable.dx) - (PI / 2.0); //rotate by PI/2 to line up with vehicle_angle angle
            transform.set_rotation_2d(velocity_angle);

            info!("dx, dy: {:?}, {:?}", movable.dx, movable.dy);
        }
    }
}