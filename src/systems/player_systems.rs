use amethyst::{core::{Time}, derive::SystemDesc, ecs::{
        Join, Read, System, SystemData, World,
        WriteStorage,
    }, input::{InputHandler, StringBindings}
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

        for (player, shield, movable, weapon) in (
            &mut players,
            &mut shields,
            &mut movables,
            &mut weapons,
        )
            .join()
        {
            player.system_adjust_cooldown.timer_update(&dt);
            if player.system_adjust_cooldown.timer_active() {

                let (
                    adjust_reset_system,
                    adjust_shield_system,
                    adjust_thrust_system,
                    adjust_weapon_system,
                ) = get_player_systems_controller_input(&player.id, &input);

                let mut adjust_attempted: bool = false;

                if adjust_reset_system {
                    shield.power.reset();
                    movable.power.reset();
                    weapon.power.reset();
                    adjust_attempted = true;
                }

                if adjust_shield_system {
                    shield.power.up(movable.power.down() + weapon.power.down());
                    adjust_attempted = true;
                }

                if adjust_thrust_system {
                    movable.power.up(shield.power.down() + weapon.power.down());
                    adjust_attempted = true;
                }

                if adjust_weapon_system {
                    weapon.power.up(shield.power.down() + movable.power.down());
                    adjust_attempted = true;
                }

                
                if adjust_attempted {
                    player.system_adjust_cooldown.timer_reset();

                    log::info!("p1 s:{:?}, t:{:?}, w:{:?}", shield.power, movable.power, weapon.power);
                }
            }
        }
    }
}


fn get_player_systems_controller_input(player_id: &usize, input: &Read<InputHandler<StringBindings>>) -> (bool, bool, bool, bool) {
    // Get Controller Input for each Player
    let adjust_reset_system = match player_id {
        0 => input.action_is_down("p1_adjust_reset_system"),
        1 => input.action_is_down("p2_adjust_reset_system"),
        2 => input.action_is_down("p3_adjust_reset_system"),
        3 => input.action_is_down("p4_adjust_reset_system"),
        _ => None,
    };

    let adjust_shield_system = match player_id {
        0 => input.action_is_down("p1_adjust_shield_system"),
        1 => input.action_is_down("p2_adjust_shield_system"),
        2 => input.action_is_down("p3_adjust_shield_system"),
        3 => input.action_is_down("p4_adjust_shield_system"),
        _ => None,
    };

    let adjust_thrust_system = match player_id {
        0 => input.action_is_down("p1_adjust_thrust_system"),
        1 => input.action_is_down("p2_adjust_thrust_system"),
        2 => input.action_is_down("p3_adjust_thrust_system"),
        3 => input.action_is_down("p4_adjust_thrust_system"),
        _ => None,
    };

    let adjust_weapon_system = match player_id {
        0 => input.action_is_down("p1_adjust_weapon_system"),
        1 => input.action_is_down("p2_adjust_weapon_system"),
        2 => input.action_is_down("p3_adjust_weapon_system"),
        3 => input.action_is_down("p4_adjust_weapon_system"),
        _ => None,
    };

    return (
        adjust_reset_system.unwrap_or(false),
        adjust_shield_system.unwrap_or(false),
        adjust_thrust_system.unwrap_or(false),
        adjust_weapon_system.unwrap_or(false),
    )
    
}