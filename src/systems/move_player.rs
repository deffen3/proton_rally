use amethyst::{
    core::{Time, Transform},
    derive::SystemDesc,
    ecs::{
        Join, Read, System, SystemData, World,
        WriteStorage, ReadStorage,
    },
    input::{InputHandler, StringBindings},
};

use std::f32::consts::PI;

use crate::components::{Movable, Mass, Player};

#[derive(SystemDesc, Default)]
pub struct MovePlayerSystem {
}

impl<'s> System<'s> for MovePlayerSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        WriteStorage<'s, Movable>,
        ReadStorage<'s, Mass>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        Read<'s, InputHandler<StringBindings>>, //<MovementBindingTypes>
    );

    fn setup(&mut self, _world: &mut World) {
    }

    fn run(
        &mut self,
        (
            players,
            mut movables,
            masses,
            mut transforms,
            time,
            input,
        ): Self::SystemData,
    ) {
        let dt = time.delta_seconds();

        for (player, mut movable, mass, transform) in (
            &players,
            &mut movables,
            &masses,
            &mut transforms,
        )
            .join()
        {
            let powered_max_accel_force = movable.max_accel_force * (movable.power.get_power_pct());

            let auto_decel_force = powered_max_accel_force / 6.0; //applied when no controller input detected
            let friction_decel_force = powered_max_accel_force / 20.0; //applied always, mass cancels out
            let air_friction_decel_force = powered_max_accel_force / 15_000.0; //applied always, based on velocity squared

            let sq_vel = movable.dx.powi(2) + movable.dy.powi(2);

            let vel_angle = movable.dy.atan2(movable.dx) - (PI / 2.0); //rotate by PI/2 to line up with vehicle_angle angle
            let vel_x_comp = -vel_angle.sin(); //left is -, right is +
            let vel_y_comp = vel_angle.cos(); //up is +, down is -


            // Get Controller Input for each Player
            let mut player_accel_x_pct = match player.id {
                0 => input.axis_value("p1_move_x"),
                1 => input.axis_value("p2_move_x"),
                2 => input.axis_value("p3_move_x"),
                3 => input.axis_value("p4_move_x"),
                _ => None,
            }.unwrap_or(0.0);

            // keyboard override for p1 for development
            if player_accel_x_pct == 0.0 {
                player_accel_x_pct = match player.id {
                    0 => input.axis_value("p1kb_move_x"),
                    _ => None,
                }.unwrap_or(0.0);
            }

            let mut player_accel_y_pct = match player.id {
                0 => input.axis_value("p1_move_y"),
                1 => input.axis_value("p2_move_y"),
                2 => input.axis_value("p3_move_y"),
                3 => input.axis_value("p4_move_y"),
                _ => None,
            }.unwrap_or(0.0);

            // keyboard override for p1 for development
            if player_accel_y_pct == 0.0 {
                player_accel_y_pct = match player.id {
                    0 => input.axis_value("p1kb_move_y"),
                    _ => None,
                }.unwrap_or(0.0);
            }

            let mut player_input: bool = false;

            // Apply Control Accelerations
            if player_accel_x_pct.abs() > 0.0 {
                player_input = true;
                movable.dx += (powered_max_accel_force * player_accel_x_pct)/mass.mass  * dt;
            }
            else if movable.dx.abs() > 0.00001 {
                movable.dx -= auto_decel_force/mass.mass * movable.dx.signum() * dt;
            }
            else {
                movable.dx = 0.0;
            }

            if player_accel_y_pct.abs() > 0.0 {
                player_input = true;
                movable.dy += (powered_max_accel_force * player_accel_y_pct)/mass.mass  * dt;
            }
            else if movable.dy.abs() > 0.00001 {
                movable.dy -= auto_decel_force/mass.mass * movable.dy.signum() * dt;
            }
            else {
                movable.dy = 0.0;
            }

            // Apply Frictions
            movable.dx -= friction_decel_force * movable.dx.signum() * dt;
            movable.dy -= friction_decel_force * movable.dy.signum() * dt;

            let air_friction_decel_force = (air_friction_decel_force * sq_vel)/mass.mass;
            movable.dx -= air_friction_decel_force * vel_x_comp * dt;
            movable.dy -= air_friction_decel_force * vel_y_comp * dt;

            // Apply physics updates to Transform
            transform.prepend_translation_x(movable.dx * dt);
            transform.prepend_translation_y(movable.dy * dt);

            if player_input == true {
                transform.set_rotation_2d(vel_angle);
            }
        }
    }
}