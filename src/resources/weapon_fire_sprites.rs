use amethyst::{
    ecs::prelude::World,
    renderer::{SpriteRender},
};

#[derive(Clone)]
pub struct WeaponFireResource {
    /// The render that locates the sprite in a sprite sheet resource
    pub player_1_weapon_fire: SpriteRender,
    pub player_2_weapon_fire: SpriteRender,
    pub player_3_weapon_fire: SpriteRender,
    pub player_4_weapon_fire: SpriteRender,
}

pub fn initialize_weapon_fire_resource(
    world: &mut World,
    sprite_sheet_handle: &Vec<SpriteRender>,
) -> () {
    let resource = WeaponFireResource {
        player_1_weapon_fire: sprite_sheet_handle[7].clone(),
        player_2_weapon_fire: sprite_sheet_handle[8].clone(),
        player_3_weapon_fire: sprite_sheet_handle[9].clone(),
        player_4_weapon_fire: sprite_sheet_handle[10].clone(),
    };
    world.insert(resource.clone());
}
