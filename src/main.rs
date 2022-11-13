use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::{application_dir, application_root_dir, fps_counter::FpsCounterBundle}
};

use std::fs::File;
use std::path::PathBuf;

use serde::de::DeserializeOwned;

mod state;

mod entities;
mod components;
mod systems;
mod resources;


fn load_ron_asset<T: DeserializeOwned>(path: &[&str]) -> T {
    let mut path_buf = PathBuf::from("assets");
    path_buf.extend(path);
    let path = application_dir(path_buf).expect("Failed to find application directory");

    let file = File::open(&path).expect(&format!("Failed to open file: {:?}", path));

    ron::de::from_reader(file).expect("Failed to load config")
}


fn main() -> amethyst::Result<()> {
    //amethyst::start_logger(Default::default());
    env_logger::init();

    let app_root = application_root_dir()?;

    let resources = app_root.join("assets");
    let display_config = app_root.join("config/display_config.ron");
    let key_bindings_path = app_root.join("config/bindings_controller.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(&key_bindings_path)?,
        )?
        .with_bundle(FpsCounterBundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)?
                        .with_clear([0.22, 0.22, 0.22, 1.0]),
                )
                .with_plugin(RenderUi::default())
                .with_plugin(RenderFlat2D::default()),
        )?;

    let mut game = Application::new(resources, state::MyState::default(), game_data)?;
    game.run();

    Ok(())
}
