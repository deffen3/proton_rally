use amethyst::{core::{Time, Transform, components::Parent}, derive::SystemDesc, ecs::{
        Join, Read, ReadExpect, System, SystemData, World,
        WriteStorage, ReadStorage, Entities,
    }, input::{InputHandler, StringBindings}, renderer::SpriteRender};

use std::f32::consts::PI;
use std::collections::HashMap;

use crate::components::{AimControlState, Player, Shield, ShieldAimChild, Weapon, WeaponAimChild};
use crate::resources::ShieldPowerResource;

#[derive(SystemDesc, Default)]
pub struct AimWeaponSystem {
}

impl<'s> System<'s> for AimWeaponSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Player>,
        WriteStorage<'s, Weapon>,
        WriteStorage<'s, WeaponAimChild>,
        WriteStorage<'s, Shield>,
        WriteStorage<'s, ShieldAimChild>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SpriteRender>,
        ReadExpect<'s, ShieldPowerResource>,
        ReadStorage<'s, Parent>,
        Read<'s, Time>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn setup(&mut self, _world: &mut World) {
    }

    fn run(
        &mut self,
        (
            entities,
            mut players,
            mut weapons,
            mut weapon_aims,
            mut shields,
            mut shield_aims,
            mut transforms,
            mut sprites,
            shield_power_resource,
            parents,
            time,
            input,
        ): Self::SystemData,
    ) {
        let dt = time.delta_seconds();

        //Find the angles for the parent player body, so this can be subtracted out of the weapon angle later
        let mut id_match_weapon_angles: HashMap<u32, (f32, Option<f32>)> = HashMap::new();
        let mut id_match_shield_angles: HashMap<u32, (f32, Option<f32>)> = HashMap::new();
        let mut id_match_shield_power_sprites: HashMap<u32, f32> = HashMap::new();

        for (entity, player, weapon, shield, transform) in (
            &entities,
            &mut players,
            &mut weapons,
            &mut shields,
            &mut transforms,
        )
            .join()
        {
            let player_id = entity.id();

            let player_rotation = transform.rotation();
            let (_, _, player_angle) = player_rotation.euler_angles();

            // Get Controller Input for each Player
            let player_aim_shield_state = match player.id {
                0 => input.action_is_down("p1_aim_shield_state"),
                1 => input.action_is_down("p2_aim_shield_state"),
                2 => input.action_is_down("p3_aim_shield_state"),
                3 => input.action_is_down("p4_aim_shield_state"),
                _ => None,
            };

            let player_aim_weapon_state = match player.id {
                0 => input.action_is_down("p1_aim_weapon_state"),
                1 => input.action_is_down("p2_aim_weapon_state"),
                2 => input.action_is_down("p3_aim_weapon_state"),
                3 => input.action_is_down("p4_aim_weapon_state"),
                _ => None,
            };
            
  
            player.aim_mode_cooldown.timer_update(&dt);
            let aim_mode_cooldown_ready = player.aim_mode_cooldown.timer_active();

            match (player_aim_shield_state, player_aim_weapon_state) {
                (Some(shield_state), Some(weapon_state)) if weapon_state == true && shield_state == true => {
                    player.aim_control_set_state(AimControlState::Locked);
                    player.aim_mode_cooldown.timer_reset();
                },
                (Some(shield_state), Some(weapon_state)) 
                        if aim_mode_cooldown_ready && weapon_state == false && shield_state == true => {
                    player.aim_control_set_state(AimControlState::Shield);
                },
                (Some(shield_state), Some(weapon_state)) 
                        if aim_mode_cooldown_ready && weapon_state == true && shield_state == false => {
                    player.aim_control_set_state(AimControlState::Weapon);
                },
                _ => {}
            }           

            
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

            let aim_angle = match (player_aim_x, player_aim_y) {
                (Some(aim_x), Some(aim_y)) if aim_x.abs() + aim_y.abs() > 0.75 => {
                    Some(aim_y.atan2(aim_x) - (PI / 2.0))
                },
                (_, _) => None, // do nothing to aim weapon, just update for player base angle
            };

            match (aim_angle, player.aim_control_weapon_active(), player.aim_control_shield_active()) {
                (Some(aim_angle), true, true) => {
                    id_match_weapon_angles.insert(player_id, (player_angle, Some(aim_angle)));
                    weapon.angle = aim_angle;

                    id_match_shield_angles.insert(player_id, (player_angle, Some(aim_angle)));
                    shield.angle = aim_angle;
                }
                (Some(aim_angle), true, _) => {
                    id_match_weapon_angles.insert(player_id, (player_angle, Some(aim_angle)));
                    weapon.angle = aim_angle;
                    
                    id_match_shield_angles.insert(player_id, (player_angle, None));
                }
                (Some(aim_angle), _, true) => {
                    id_match_weapon_angles.insert(player_id, (player_angle, None));
                    
                    id_match_shield_angles.insert(player_id, (player_angle, Some(aim_angle)));
                    shield.angle = aim_angle;
                }
                _ => {
                    id_match_weapon_angles.insert(player_id, (player_angle, None));
                    
                    id_match_shield_angles.insert(player_id, (player_angle, None));
                }
            }


            
            id_match_shield_power_sprites.insert(player_id, shield.power.get_power_pct());
            
        }


        for (weapon_aim, parent, transform) in (
            &mut weapon_aims,
            &parents,
            &mut transforms,
        )
            .join()
        {
            let parent_id = parent.entity.id();

            let (player_base_angle, aim_angle) = id_match_weapon_angles
                .get(&parent_id)
                .unwrap_or(&(0.0, None));
            
            match aim_angle {
                Some(angle) => weapon_aim.angle = *angle,
                None => ()
            };

            transform.set_rotation_2d(-player_base_angle + weapon_aim.angle);
        }


        for (shield_aim, parent, sprite, transform) in (
            &mut shield_aims,
            &parents,
            &mut sprites,
            &mut transforms,
        )
            .join()
        {
            let parent_id = parent.entity.id();

            let (player_base_angle, aim_angle) = id_match_shield_angles
                .get(&parent_id)
                .unwrap_or(&(0.0, None));
            
            match aim_angle {
                Some(angle) => shield_aim.angle = *angle,
                None => ()
            };

            transform.set_rotation_2d(-player_base_angle + shield_aim.angle);


            let shield_power = id_match_shield_power_sprites.get(&parent_id);

            match shield_power {
                Some(s) if *s < 0.333 => {
                    *sprite = shield_power_resource.shield_off.clone();
                },
                Some(s) if *s < 0.666 => {
                    *sprite = shield_power_resource.shield_30deg.clone();
                },
                Some(s) if *s < 1.0 => {
                    *sprite = shield_power_resource.shield_60deg.clone();
                },
                Some(s) if *s < 1.666 => {
                    *sprite = shield_power_resource.shield_90deg.clone();
                },
                Some(s) if *s < 2.333 => {
                    *sprite = shield_power_resource.shield_180deg.clone();
                },
                Some(s) if *s < 3.0 => {
                    *sprite = shield_power_resource.shield_270deg.clone();
                },
                Some(s) if *s >= 3.0 => {
                    *sprite = shield_power_resource.shield_360deg.clone();
                },
                _ => {}
            }
        }
    }
}