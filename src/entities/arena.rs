use amethyst::{
    core::{transform::Transform, math::Vector3},
    ecs::prelude::{World},
    prelude::*,
    renderer::{SpriteRender},
};

use crate::load_ron_asset;

use crate::components::{ArenaStoreResource, Arena, ArenaElement, ArenaElementKinds, Hitbox, HitboxShape};


pub fn build_arena_store(world: &mut World) {
    world.insert(ArenaStoreResource {
        properties: load_ron_asset(&["game", "arena_properties.ron"]),
    });
}


pub fn intialize_arena(
    world: &mut World,
    arena_properties: &Arena,
    sprite_sheet_handle: &Vec<SpriteRender>,
    texture_sheet_handle: &Vec<SpriteRender>,
) {
    for arena_floor in arena_properties.floor.iter() {
        let sprite_scale_mult = 64.0;
        let x_scale = arena_floor.width / sprite_scale_mult;
        let y_scale = arena_floor.height / sprite_scale_mult;

        let mut floor_transform = Transform::default();
        floor_transform.set_translation_xyz(arena_floor.x, arena_floor.y, -0.05);
        floor_transform.set_scale(Vector3::new(x_scale, y_scale, 0.0));

        world
            .create_entity()
            .with(floor_transform)
            .with(texture_sheet_handle[0].clone())
            .build();
    }

    for arena_element in arena_properties.arena_elements.iter() {
        let sprite_scale_mult = 50.0;
        let x_scale = arena_element.hitbox.width / sprite_scale_mult;
        let y_scale = arena_element.hitbox.height / sprite_scale_mult;

        let mut floor_transform = Transform::default();
        floor_transform.set_translation_xyz(arena_element.element.x, arena_element.element.y, -0.05);
        floor_transform.set_scale(Vector3::new(x_scale, y_scale, 0.0));

        let sprite = match arena_element.hitbox.shape {
            HitboxShape::Circle => sprite_sheet_handle[5].clone(),
            HitboxShape::Rectangle => sprite_sheet_handle[4].clone(),
        };

        match arena_element.element.kind {
            ArenaElementKinds::Wall => {
                world
                    .create_entity()
                    .with(ArenaElement{
                        kind: arena_element.element.kind,
                        x: arena_element.element.x,
                        y: arena_element.element.y})
                    .with(Hitbox{
                        width: arena_element.hitbox.width,
                        height: arena_element.hitbox.height,
                        shape: arena_element.hitbox.shape})
                    .with(floor_transform)
                    .with(sprite)
                    .build();
            },
            _ => {
                world
                    .create_entity()
                    .with(ArenaElement{
                        kind: arena_element.element.kind,
                        x: arena_element.element.x,
                        y: arena_element.element.y})
                    .with(floor_transform)
                    .with(sprite)
                    .build();
            }
        }


    }
}