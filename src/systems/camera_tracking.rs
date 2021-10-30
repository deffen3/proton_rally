use amethyst::{
    core::{transform::Transform, Time},
    derive::SystemDesc,
    ecs::{Join, Read, ReadExpect, ReadStorage, System, SystemData, World, WriteStorage},
    renderer::camera::{Camera}, //Projection
    window::ScreenDimensions,
};

use log::info;

use crate::components::{
    ArenaNames, Arena, ArenaStoreResource, Player, PlayerState,
};

const CAMERA_ZOOM_RATE: f32 = 120.0;
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
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        WriteStorage<'s, Camera>,
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
        mut transforms,
        time,
        mut cameras,
        screen_dimensions,
    ): Self::SystemData,
    ) {
        let dt = time.delta_seconds();

        let mut player_xs = Vec::<f32>::new();
        let mut player_ys = Vec::<f32>::new();

        for (player, transform) in (&players, &transforms).join() {
            if player.state == PlayerState::Active || player.state == PlayerState::InRespawn {
                let player_x = transform.translation().x;
                let player_y = transform.translation().y;

                player_xs.push(player_x);
                player_ys.push(player_y);
            }
        }

        player_xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
        player_ys.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut player_min_x: f32 = 0.0;
        let mut player_max_x: f32 = 0.0;
        let mut player_min_y: f32 = 0.0;
        let mut player_max_y: f32 = 0.0;

        if player_xs.len() > 0 {
            player_min_x = player_xs[0];
            player_max_x = player_xs[player_xs.len() - 1];
            player_min_y = player_ys[0];
            player_max_y = player_ys[player_ys.len() - 1];
        }

        //this is the extra buffer space that the camera gives
        let offset = 100.0; //was 80.0 for combat and 160.0 for race mode in old rally_game project

        player_min_x = (player_min_x - offset).max(0.0);
        player_max_x = (player_max_x + offset).min(self.arena_properties.width);
        player_min_y = (player_min_y - offset).max(-40.0);
        player_max_y = (player_max_y + offset).min(self.arena_properties.height);

        for (camera, transform) in (&mut cameras, &mut transforms).join() {
            let aspect_ratio = screen_dimensions.aspect_ratio();

            if self.init_state {
                self.init_state = false; //never goes back to true, until this system is re-dispatched

                //Standard full Arena translation
                transform.set_translation_x(self.arena_properties.width / 2.0);
                transform.set_translation_y(self.arena_properties.height / 2.0);

                //Standard full Arena Projection
                let x_delta = self.arena_properties.width;
                let y_delta = self.arena_properties.height;

                //keep aspect ratio consistent
                let target_delta = (x_delta / aspect_ratio).max(y_delta);

                let camera_left = -target_delta * aspect_ratio / 2.0;
                let camera_right = target_delta * aspect_ratio / 2.0;
                let camera_bottom = -target_delta / 2.0;
                let camera_top = target_delta / 2.0;

                *camera = Camera::orthographic(
                    camera_left,
                    camera_right,
                    camera_bottom,
                    camera_top,
                    0.0,
                    20.0,
                );

                // camera.set_projection(Projection::orthographic(
                //     camera_left,
                //     camera_right,
                //     camera_bottom,
                //     camera_top,
                //     0.0,
                //     20.0,
                // ));
            } else {
                //Update as game progresses

                // info!("{:?}", &camera);

                // let camera_projection = camera.projection().as_orthographic().unwrap();

                // let camera_bottom = camera_projection.bottom();
                // let camera_top = camera_projection.top();

                // let camera_target_x = player_min_x + (player_max_x - player_min_x) / 2.0;
                // let camera_target_y = player_min_y + (player_max_y - player_min_y) / 2.0;

                // let x_delta = player_max_x - player_min_x;
                // let y_delta = player_max_y - player_min_y;

                // //keep aspect ratio consistent
                // let target_delta = (x_delta / aspect_ratio).max(y_delta);

                // let old_delta = camera_top - camera_bottom;
                // let d_delta = (target_delta - old_delta)
                //     .min(CAMERA_ZOOM_RATE)
                //     .max(-CAMERA_ZOOM_RATE);

                // let new_delta = old_delta + d_delta * dt;
                // //let new_delta = target_delta;

                // let camera_new_left = -new_delta * aspect_ratio / 2.0;
                // let camera_new_right = new_delta * aspect_ratio / 2.0;
                // let camera_new_bottom = -new_delta / 2.0;
                // let camera_new_top = new_delta / 2.0;

                // //Updated Projection
                // *camera = Camera::orthographic(
                //     camera_new_left,
                //     camera_new_right,
                //     camera_new_bottom,
                //     camera_new_top,
                //     0.0,
                //     20.0,
                // );

                // // camera.set_projection(Projection::orthographic(
                // //     camera_new_left,
                // //     camera_new_right,
                // //     camera_new_bottom,
                // //     camera_new_top,
                // //     0.0,
                // //     20.0,
                // // ));

                // //Updated Translation
                // let camera_x = transform.translation().x;
                // let camera_y = transform.translation().y;

                // let mut dx = (camera_target_x - camera_x)
                //     .min(CAMERA_TRANSLATE_MAX_RATE)
                //     .max(-CAMERA_TRANSLATE_MAX_RATE);
                // if dx.abs() <= 0.01 {
                //     dx = 0.0;
                // }

                // let mut dy = (camera_target_y - camera_y)
                //     .min(CAMERA_TRANSLATE_MAX_RATE)
                //     .max(-CAMERA_TRANSLATE_MAX_RATE);
                // if dy.abs() <= 0.01 {
                //     dy = 0.0;
                // }

                // transform.set_translation_x(camera_x + dx * dt);
                // transform.set_translation_y(camera_y + dy * dt);
            }
        }
    }
}
