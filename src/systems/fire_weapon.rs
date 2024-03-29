use amethyst::{
    core::{Time, Transform},
    derive::SystemDesc,
    ecs::{Entities, Join, LazyUpdate, Read, ReadExpect, 
        System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::components::{Player, Weapon};
use crate::resources::WeaponFireResource;
use crate::entities::{fire_weapon};

#[derive(SystemDesc, Default)]
pub struct FireWeaponsSystem;

impl<'s> System<'s> for FireWeaponsSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Player>,
        WriteStorage<'s, Weapon>,
        WriteStorage<'s, Transform>,
        ReadExpect<'s, WeaponFireResource>,
        ReadExpect<'s, LazyUpdate>,
        Read<'s, Time>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut players,
            mut weapons,
            mut transforms,
            weapon_fire_resource,
            lazy_update,
            time,
            input,
        ): Self::SystemData,
    ) {
        let dt = time.delta_seconds();

        for (entity, player, weapon, transform) in (
            &entities,
            &mut players,
            &mut weapons,
            &mut transforms,
        )
            .join()
        {
            let primary_fire = match player.id {
                0 => input.axis_value("p1_fire"),
                1 => input.axis_value("p2_fire"),
                2 => input.axis_value("p3_fire"),
                3 => input.axis_value("p4_fire"),
                _ => None,
            };

            match (primary_fire, weapon.cooldown.timer_active(), weapon.power.is_powered()) {
                (_, _, power) if (power  == true) => {
                    //Do nothing, not even decrease cooldown timer, weapon systems are off
                }
                (_, cooldown_active, _) if (cooldown_active == true) => {
                    weapon.cooldown.timer_update(&dt);
                }
                (Some(fire), cooldown_active, _) if (fire > 0.5) & (cooldown_active == false) => {
                    fire_weapon(
                        &entities,
                        entity.id(),
                        player.id,
                        &transform,
                        &weapon,
                        &weapon_fire_resource,
                        &lazy_update,
                    );

                    weapon.cooldown.timer_reset_multiplier(1.0 / weapon.power.get_power_pct());
                }
                (_, _, _) => {}
            }
        }
    }
}