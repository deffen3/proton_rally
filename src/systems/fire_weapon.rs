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
                0 => input.action_is_down("p1_fire"),
                1 => input.action_is_down("p2_fire"),
                2 => input.action_is_down("p3_fire"),
                3 => input.action_is_down("p4_fire"),
                _ => None,
            };

            match (primary_fire, weapon.cooldown_timer) {
                (_, cooldown_timer) if (cooldown_timer >= 0.0) => {
                    weapon.cooldown_timer -= dt;
                }
                (Some(fire), cooldown_timer) if (fire == true) & (cooldown_timer <= 0.0) => {
                    fire_weapon(
                        &entities,
                        entity.id(),
                        &transform,
                        &weapon,
                        &weapon_fire_resource,
                        &lazy_update,
                    );

                    weapon.cooldown_timer = weapon.cooldown_reset;
                }
                (_, _) => ()
            }
        }
    }
}