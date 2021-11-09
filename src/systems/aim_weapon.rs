use amethyst::{
    core::{Transform, components::Parent},
    derive::SystemDesc,
    ecs::{
        Join, Read, System, SystemData, World,
        WriteStorage, ReadStorage, Entities,
    },
    input::{InputHandler, StringBindings},
};

use std::f32::consts::PI;
use std::collections::HashMap;

use crate::components::{Weapon, WeaponAimChild, Player};

#[derive(SystemDesc, Default)]
pub struct AimWeaponSystem {
}

impl<'s> System<'s> for AimWeaponSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Player>,
        WriteStorage<'s, Weapon>,
        WriteStorage<'s, WeaponAimChild>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Parent>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn setup(&mut self, _world: &mut World) {
    }

    fn run(
        &mut self,
        (
            entities,
            players,
            mut weapons,
            mut weapon_aims,
            mut transforms,
            parents,
            input,
        ): Self::SystemData,
    ) {
        //Find the angles for the parent player body, so this can be subtracted out of the weapon angle later
        let mut id_match_angles: HashMap<u32, (f32, Option<f32>)> = HashMap::new();

        for (entity, player, weapon, transform) in (
            &entities,
            &players,
            &mut weapons,
            &mut transforms,
        )
            .join()
        {
            let player_id = entity.id();

            let player_rotation = transform.rotation();
            let (_, _, player_angle) = player_rotation.euler_angles();

            // Get Controller Input for each Player
            let player_aim_x = match player.id {
                0 => input.axis_value("p1_aim_x"),
                1 => input.axis_value("p2_aim_x"),
                2 => input.axis_value("p3_aim_x"),
                3 => input.axis_value("p4_aim_x"),
                _ => None,
            };

            // keyboard override for p1 for development
            // let player_aim_x = match player_aim_x {
            //     None => input.axis_value("p1kb_aim_x"),
            //     Some(aim_x) => Some(aim_x),
            // };

            let player_aim_y = match player.id {
                0 => input.axis_value("p1_aim_y"),
                1 => input.axis_value("p2_aim_y"),
                2 => input.axis_value("p3_aim_y"),
                3 => input.axis_value("p4_aim_y"),
                _ => None,
            };

            // keyboard override for p1 for development
            // let player_aim_y = match player_aim_y {
            //     None => input.axis_value("p1kb_aim_y"),
            //     Some(aim_y) => Some(aim_y),
            // };

            let weapon_aim_angle = match (player_aim_x, player_aim_y) {
                (Some(aim_x), Some(aim_y)) if aim_x.abs() + aim_y.abs() > 0.75 => {
                    Some(aim_y.atan2(aim_x) - (PI / 2.0))
                },
                (_, _) => None, // do nothing to aim weapon, just update for player base angle
            };

            if let Some(new_weapon_aim_angle) = weapon_aim_angle {
                weapon.angle = new_weapon_aim_angle;
            }

            id_match_angles.insert(player_id, (player_angle, weapon_aim_angle));
        }


        for (weapon_aim, parent, transform) in (
            &mut weapon_aims,
            &parents,
            &mut transforms,
        )
            .join()
        {
            let parent_id = parent.entity.id();

            let (player_base_angle, weapon_aim_angle) = id_match_angles
                .get(&parent_id)
                .unwrap_or(&(0.0, None));
            
            match weapon_aim_angle {
                Some(angle) => weapon_aim.angle = *angle,
                None => ()
            };

            transform.set_rotation_2d(-player_base_angle + weapon_aim.angle);
        }
    }
}