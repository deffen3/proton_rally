use amethyst::{
    ecs::prelude::World,
    renderer::{SpriteRender},
};

#[derive(Clone)]
pub struct ShieldPowerResource {
    /// The render that locates the sprite in a sprite sheet resource
    pub shield_off: SpriteRender,
    pub shield_30deg: SpriteRender,
    pub shield_60deg: SpriteRender,
    pub shield_90deg: SpriteRender,
    pub shield_180deg: SpriteRender,
    pub shield_270deg: SpriteRender,
    pub shield_360deg: SpriteRender,
}

pub fn initialize_shield_power_resource(
    world: &mut World,
    sprite_sheet_handle: &Vec<SpriteRender>,
) -> () {
    let resource = ShieldPowerResource {
        shield_off: sprite_sheet_handle[17].clone(),
        shield_30deg: sprite_sheet_handle[11].clone(),
        shield_60deg: sprite_sheet_handle[12].clone(),
        shield_90deg: sprite_sheet_handle[13].clone(),
        shield_180deg: sprite_sheet_handle[14].clone(),
        shield_270deg: sprite_sheet_handle[15].clone(),
        shield_360deg: sprite_sheet_handle[16].clone(),
    };
    world.insert(resource.clone());
}
