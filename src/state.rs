use amethyst::{
    assets::{Loader},
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    ui::{
        Anchor, FontHandle, LineMode, TtfFormat, UiImage, UiText,
        UiTransform,
    },
    ecs::prelude::{DispatcherBuilder, Dispatcher},
    renderer::{Camera},
    utils::removal::Removal,
};

use crate::{components::{WeaponAimChild, WeaponFire}, entities::{
    build_arena_store, intialize_arena, initialize_camera, intialize_player}, systems::PlayerSystemsSystem};
use crate::components::{
    ArenaNames, ArenaStoreResource, Arena, ArenaElement,
    CameraOrthoEdges, 
    Movable, Mass, Player, Hitbox, Weapon, Shield, ShieldAimChild};
use crate::systems::{
    CameraTrackingSystem, 
    MovePlayerSystem, AimWeaponSystem, FireWeaponsSystem, MoveWeaponFireSystem,
    HitboxCollisionDetection, HitboxImmovableCollisionDetection};
use crate::resources::{
    load_sprites, load_world_textures, 
    initialize_weapon_fire_resource,
    initialize_shield_power_resource};


#[derive(Default)]
pub struct MyState<'a, 'b> {
    /// The `State` specific `Dispatcher`, containing `System`s only relevant for this `State`.
    dispatcher: Option<Dispatcher<'a, 'b>>
}


impl<'a, 'b> SimpleState for MyState<'a, 'b> {
    // Here, we define hooks that will be called throughout the lifecycle of our game state.
    //
    // In this example, `on_start` is used for initializing entities
    // and `handle_state` for managing the state transitions.
    //
    // For more state lifecycle hooks, see:
    // https://book.amethyst.rs/stable/concepts/state.html#life-cycle

    /// The state is initialized with:
    /// - a camera centered in the middle of the screen.
    /// - 3 sprites places around the center.
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        build_arena_store(world);

        world.register::<Camera>();
        world.register::<CameraOrthoEdges>();
        world.register::<ArenaElement>();
        world.register::<Player>();
        world.register::<Movable>();
        world.register::<Mass>();
        world.register::<Hitbox>();
        world.register::<Shield>();
        world.register::<ShieldAimChild>();
        world.register::<Weapon>();
        world.register::<WeaponFire>();
        world.register::<WeaponAimChild>();
        world.register::<Removal<u32>>();


        let arena_name = ArenaNames::StandardCombat;
        let arena_properties;
        {
            let fetched_arena_store = world.try_fetch::<ArenaStoreResource>();

            if let Some(arena_store) = fetched_arena_store {
                arena_properties = match arena_store.properties.get(&arena_name) {
                    Some(arena_props_get) => (*arena_props_get).clone(),
                    _ => Arena::default(),
                };
            } else {
                arena_properties = Arena::default();
            }
        }

        // Place the camera
        initialize_camera(world, &arena_properties);

        // Load our sprites and display them
        let sprites = load_sprites(world);
        let world_textures = load_world_textures(world);

        initialize_weapon_fire_resource(world, &sprites);
        initialize_shield_power_resource(world, &sprites);

        intialize_arena(world, &arena_properties, &sprites, &world_textures);
        intialize_player(world, &arena_properties, &sprites);

        create_ui_example(world);


        // Create the `DispatcherBuilder` and register some `System`s that should only run for this `State`.
        let mut dispatcher_builder = DispatcherBuilder::new();

        dispatcher_builder.add(
            CameraTrackingSystem{
                arena_name: arena_name,
                arena_properties: arena_properties,
                init_state: true},
            "camera_tracking_system",
            &[],
        );
        dispatcher_builder.add(
            MovePlayerSystem::default(), "move_player_system", &[]);
        dispatcher_builder.add(
            AimWeaponSystem::default(), "aim_weapon_system", &[]);
        dispatcher_builder.add(
            FireWeaponsSystem::default(), "fire_weapon_system", &[]);
        dispatcher_builder.add(
            HitboxCollisionDetection::default(), "hitbox_collision_system", &[]);
        dispatcher_builder.add(
            HitboxImmovableCollisionDetection::default(), "hitbox_immovable_collision_system", &[]);
        dispatcher_builder.add(
            MoveWeaponFireSystem::default(), "move_weapon_fire_system", &[]);
        dispatcher_builder.add(
            PlayerSystemsSystem::default(), "player_systems_system", &[]);

        
        // Build and setup the `Dispatcher`.
        let mut dispatcher = dispatcher_builder.build();
        dispatcher.setup(world);

        self.dispatcher = Some(dispatcher);
    }

    /// The following events are handled:
    /// - The game state is quit when either the close button is clicked or when the escape key is pressed.
    /// - Any other keypress is simply logged to the console.
    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            // Check if the window should be closed
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }
        }

        // Keep going
        Trans::None
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if let Some(dispatcher) = self.dispatcher.as_mut() {
            dispatcher.dispatch(&data.world);
        }

        Trans::None
    }
}



/// Creates a simple UI background and a UI text label
/// This is the pure code only way to create UI with amethyst.
pub fn create_ui_example(world: &mut World) {
    // this creates the simple gray background UI element.
    let _ui_background = world
        .create_entity()
        .with(UiImage::SolidColor([0.6, 0.1, 0.2, 1.0]))
        .with(UiTransform::new(
            "".to_string(),
            Anchor::TopLeft,
            Anchor::TopLeft,
            0.,
            0.,
            0.,
            220.,
            40.,
        ))
        .build();

    // This simply loads a font from the asset folder and puts it in the world as a resource,
    // we also get a ref to the font that we then can pass to the text label we crate later.
    let font: FontHandle = world.read_resource::<Loader>().load(
        "fonts/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );

    // This creates the actual label and places it on the screen.
    // Take note of the z position given, this ensures the label gets rendered above the background UI element.
    world
        .create_entity()
        .with(UiTransform::new(
            "".to_string(),
            Anchor::TopLeft,
            Anchor::TopLeft,
            5.,
            -5.,
            1.,
            200.,
            50.,
        ))
        .with(UiText::new(
            font,
            "Proton Rally!".to_string(),
            [1., 1., 1., 1.],
            30.,
            LineMode::Single,
            Anchor::TopLeft,
        ))
        .build();
}