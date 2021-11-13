use amethyst::{
    core::{Time, Transform},
    derive::SystemDesc,
    ecs::{Entities, Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
};

use crate::components::{
    WeaponFire, Movable,
};

#[derive(SystemDesc, Default)]
pub struct MoveWeaponFireSystem {
}

impl<'s> System<'s> for MoveWeaponFireSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, WeaponFire>,
        ReadStorage<'s, Movable>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn setup(&mut self, _world: &mut World) {
    }

    fn run(
        &mut self,
        (
            entities,
            weapon_fires,
            movables,
            mut transforms,
            time
        ): Self::SystemData,
    ) {
        let dt = time.delta_seconds();

        for (_entity, _weapon_fire, movable, transform) in (
            &entities,
            &weapon_fires,
            &movables,
            &mut transforms,
        )
            .join()
        {
            // Apply physics updates to Transform
            transform.prepend_translation_x(movable.dx * dt);
            transform.prepend_translation_y(movable.dy * dt);
        }
    }
}