use amethyst::{
    core::{transform::Transform, Time},
    derive::SystemDesc,
    ecs::{Join, Read, ReadExpect, ReadStorage, System, SystemData, World, WriteStorage},
    renderer::camera::{Camera},
    window::ScreenDimensions,
};

use crate::components::{
    ArenaNames, Arena, ArenaStoreResource, CameraOrthoEdges, CameraPlayerBounds, Player, PlayerState, Movable,
};


const CAMERA_TRANSLATE_MAX_RATE: f32 = 100.0;

#[derive(SystemDesc)]
pub struct CameraTrackingSystem {
    pub arena_name: ArenaNames,
    pub arena_properties: Arena,
    pub init_state: bool,
}

impl<'s> System<'s> for CameraTrackingSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        ReadStorage<'s, Movable>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        WriteStorage<'s, Camera>,
        WriteStorage<'s, CameraOrthoEdges>,
        ReadExpect<'s, ScreenDimensions>,
    );

    fn setup(&mut self, world: &mut World) {
        self.init_state = true;

        {
            let fetched_arena_store = world.try_fetch::<ArenaStoreResource>();
            if let Some(arena_store) = fetched_arena_store {
                self.arena_properties = match arena_store.properties.get(&self.arena_name) {
                    Some(arena_props_get) => (*arena_props_get).clone(),
                    _ => Arena::default(),
                };
            } else {
                self.arena_properties = Arena::default();
            }
        }
    }

    fn run(
        &mut self,
        (
        players,
        movables,
        mut transforms,
        time,
        mut cameras,
        mut camera_orthos,
        screen_dimensions,
    ): Self::SystemData,
    ) {
        let dt = time.delta_seconds();

        let mut player_bounds = CameraPlayerBounds::new();

        for (player, movable, transform) in (&players, &movables, &transforms).join() {
            if player.player_state_in_game() {
                player_bounds.check_player(
                    transform.translation().x,
                    transform.translation().y,
                    movable.dx.abs(),
                    movable.dy.abs(),
                )
            }
        }

        player_bounds.calc_bounds_on_players_and_arena(
            self.arena_properties.width,
            self.arena_properties.height
        );

        for (camera, camera_ortho, transform) in (&mut cameras, &mut camera_orthos, &mut transforms).join() {
            let aspect_ratio = screen_dimensions.aspect_ratio();

            if self.init_state {
                self.init_state = false; //never goes back to true, until this system is re-dispatched

                //Standard full Arena translation
                transform.set_translation_x(self.arena_properties.width / 2.0);
                transform.set_translation_y(self.arena_properties.height / 2.0);

                //keep aspect ratio consistent
                camera_ortho.init_edges_keeping_aspect_ratio(
                    aspect_ratio, 
                    self.arena_properties.width,
                    self.arena_properties.height
                );

                *camera = Camera::orthographic(
                    camera_ortho.left,
                    camera_ortho.right,
                    camera_ortho.bottom,
                    camera_ortho.top,
                    0.0,
                    20.0,
                );
            } else {
                //Update as game progresses

                // Keep aspect ratio consistent
                camera_ortho.calculate_edges_keeping_aspect_ratio(
                    aspect_ratio, 
                    player_bounds.get_span_x(),
                    player_bounds.get_span_y(),
                    dt,
                );

                // Updated Projection
                *camera = Camera::orthographic(
                    camera_ortho.left,
                    camera_ortho.right,
                    camera_ortho.bottom,
                    camera_ortho.top,
                    0.0,
                    20.0,
                );

                // Update Camera Translation
                let camera_x = transform.translation().x;
                let camera_y = transform.translation().y;

                let mut camera_dx = (player_bounds.get_target_x() - camera_x)
                    .min(CAMERA_TRANSLATE_MAX_RATE)
                    .max(-CAMERA_TRANSLATE_MAX_RATE);
                if camera_dx.abs() <= 0.01 {
                    camera_dx = 0.0;
                }

                let mut camera_dy = (player_bounds.get_target_y() - camera_y)
                    .min(CAMERA_TRANSLATE_MAX_RATE)
                    .max(-CAMERA_TRANSLATE_MAX_RATE);
                if camera_dy.abs() <= 0.01 {
                    camera_dy = 0.0;
                }

                transform.set_translation_x(camera_x + camera_dx * dt);
                transform.set_translation_y(camera_y + camera_dy * dt);
            }
        }
    }
}
