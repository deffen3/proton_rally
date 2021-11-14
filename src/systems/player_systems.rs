use amethyst::{
    core::{Time},
    derive::SystemDesc,
    ecs::{
        Join, Read, System, SystemData, World,
        WriteStorage,
    },
    input::{InputHandler, StringBindings},
};

use crate::components::{Movable, Player, Shield, Weapon};

#[derive(SystemDesc, Default)]
pub struct PlayerSystemsSystem {
}

impl<'s> System<'s> for PlayerSystemsSystem {
    type SystemData = (
        WriteStorage<'s, Player>,
        WriteStorage<'s, Shield>,
        WriteStorage<'s, Movable>,
        WriteStorage<'s, Weapon>,
        Read<'s, Time>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn setup(&mut self, _world: &mut World) {
    }

    fn run(
        &mut self,
        (
            mut players,
            mut shields,
            mut movables,
            mut weapons,
            time,
            input,
        ): Self::SystemData,
    ) {
        let dt = time.delta_seconds();

        for (mut player, mut shield, mut movable, mut weapon) in (
            &mut players,
            &mut shields,
            &mut movables,
            &mut weapons,
        )
            .join()
        {
            if player.system_adjust_cooldown_timer > 0.0 {
                player.system_adjust_cooldown_timer -= dt;
            }
            else {
                // Get Controller Input for each Player
                let adjust_shield_system = match player.id {
                    0 => input.action_is_down("p1_adjust_shield_system"),
                    1 => input.action_is_down("p2_adjust_shield_system"),
                    2 => input.action_is_down("p3_adjust_shield_system"),
                    3 => input.action_is_down("p4_adjust_shield_system"),
                    _ => None,
                };
                let adjust_thrust_system = match player.id {
                    0 => input.action_is_down("p1_adjust_thrust_system"),
                    1 => input.action_is_down("p2_adjust_thrust_system"),
                    2 => input.action_is_down("p3_adjust_thrust_system"),
                    3 => input.action_is_down("p4_adjust_thrust_system"),
                    _ => None,
                };
                let adjust_weapon_system = match player.id {
                    0 => input.action_is_down("p1_adjust_weapon_system"),
                    1 => input.action_is_down("p2_adjust_weapon_system"),
                    2 => input.action_is_down("p3_adjust_weapon_system"),
                    3 => input.action_is_down("p4_adjust_weapon_system"),
                    _ => None,
                };
                let adjust_reset_system = match player.id {
                    0 => input.action_is_down("p1_adjust_reset_system"),
                    1 => input.action_is_down("p2_adjust_reset_system"),
                    2 => input.action_is_down("p3_adjust_reset_system"),
                    3 => input.action_is_down("p4_adjust_reset_system"),
                    _ => None,
                };

                let adjust_attempted = match (adjust_shield_system, adjust_thrust_system, adjust_weapon_system, adjust_reset_system) {
                    (_, _, _, Some(r)) if r == true => {
                        shield.power = shield.power_base;
                        movable.power = movable.power_base;
                        weapon.power = weapon.power_base;

                        true
                    },
                    (Some(s), _, _, _) if s == true => {
                        if movable.power >= 3 {
                            movable.power -= 3;
                            shield.power += 3;
                        }
                        if weapon.power >= 3 {
                            weapon.power -= 3;
                            shield.power += 3;
                        }

                        true
                    },
                    (_, Some(t), _, _) if t == true => {
                        if shield.power >= 3 {
                            shield.power -= 3;
                            movable.power += 3;
                        }
                        if weapon.power >= 3 {
                            weapon.power -= 3;
                            movable.power += 3;
                        }

                        true
                    },
                    (_, _, Some(w), _) if w == true => {
                        if movable.power >= 3 {
                            movable.power -= 3;
                            weapon.power += 3;
                        }
                        if shield.power >= 3 {
                            shield.power -= 3;
                            weapon.power += 3;
                        }

                        true
                    },
                    _ => {false}
                };

                if adjust_attempted {
                    player.system_adjust_cooldown_timer = player.system_adjust_cooldown_reset;

                    log::info!("p1 s:{:?}, t:{:?}, w:{:?}", shield.power, movable.power, weapon.power);
                }
            }
        }
    }
}