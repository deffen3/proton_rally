use amethyst::{
    core::{transform::Transform, math::Vector3},
    ecs::prelude::{Entity, World},
    prelude::*,
    renderer::{SpriteRender},
};

use crate::load_ron_asset;

use std::f32::consts::PI;

use crate::components::{ArenaStoreResource, Arena, ArenaNames, ArenaElement, ArenaElementKinds, Hitbox, HitboxShape};


pub fn build_arena_store(world: &mut World) {
    world.insert(ArenaStoreResource {
        properties: load_ron_asset(&["game", "arena_properties.ron"]),
    });
}


pub fn intialize_arena(
    arena_name: ArenaNames,
    world: &mut World,
    sprite_sheet_handle: &Vec<SpriteRender>,
    texture_sheet_handle: &Vec<SpriteRender>,
) {    
    let arena_properties = match world.try_fetch::<ArenaStoreResource>() {
        Some(arena_store) => {
            match arena_store.properties.get(&arena_name) {
                Some(arena_props_get) => (*arena_props_get).clone(),
                _ => Arena::default(),
            }
        },
        _ => Arena::default(),
    };


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
            HitboxShape::Circle => sprite_sheet_handle[2].clone(),
            HitboxShape::Rectangle => sprite_sheet_handle[1].clone(),
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

    // let x = 0.0;
    // let y = 0.0;
    // let angle = 0.0;
    // let scale = 5.0;
    // let width = 10.0 * scale;
    // let height = 50.0 * scale;

    // let mut arena_element_transform = Transform::default();
    // arena_element_transform.set_rotation_2d(angle);
    // arena_element_transform.set_translation_xyz(x, y, 0.0);
    // arena_element_transform.set_scale(Vector3::new(
    //     1.0 * scale,
    //     1.0 * scale,
    //     0.0,
    // ));

    // world
    //     .create_entity()
    //     .with(arena_element_transform)
    //     .with(sprite_sheet_handle[1].clone())
    //     .with(ArenaElement::new(x, y, angle))
    //     .with(Hitbox::new(width, height, HitboxShape::Rectangle))
    //     .build();


    // let x = 500.0;
    // let y = 400.0;
    // let angle = PI/2.0;
    // let scale = 5.0;
    // let width = 50.0 * scale; //rotated
    // let height = 10.0 * scale; //rotated

    // let mut arena_element_transform = Transform::default();
    // arena_element_transform.set_rotation_2d(angle);
    // arena_element_transform.set_translation_xyz(x, y, 0.0);
    // arena_element_transform.set_scale(Vector3::new(
    //     1.0 * scale,
    //     1.0 * scale,
    //     0.0,
    // ));

    // world
    //     .create_entity()
    //     .with(arena_element_transform)
    //     .with(sprite_sheet_handle[1].clone())
    //     .with(ArenaElement::new(x, y, angle))
    //     .with(Hitbox::new(width, height, HitboxShape::Rectangle))
    //     .build();
}