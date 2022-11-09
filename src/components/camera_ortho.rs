use std::cmp::Ordering;

use amethyst::ecs::prelude::{Component, DenseVecStorage};

use serde::Deserialize;

// Just used to store the current orthographic projection of the camera
#[derive(Copy, Clone, Debug, PartialEq, Deserialize)]
pub struct CameraOrtho {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
}

impl Component for CameraOrtho {
    type Storage = DenseVecStorage<Self>;
}


pub struct CameraPlayerBounds {
    xs: Vec<f32>,
    ys: Vec<f32>,
    dxs: Vec<f32>,
    dys: Vec<f32>,
    min_x: f32,
    max_x: f32,
    min_y: f32,
    max_y: f32,
    max_abs_dx: f32,
    max_abs_dy: f32,
}

impl CameraPlayerBounds {
    pub fn new() -> CameraPlayerBounds {
        CameraPlayerBounds {
            xs: Vec::<f32>::new(),
            ys: Vec::<f32>::new(),
            dxs: Vec::<f32>::new(),
            dys: Vec::<f32>::new(),
            min_x: 0.0,
            max_x: 0.0,
            min_y: 0.0,
            max_y: 0.0,
            max_abs_dx: 0.0,
            max_abs_dy: 0.0,
        }
    }

    pub fn check_player(&mut self, x:f32, y:f32, dx:f32, dy:f32) {
        self.xs.push(x);
        self.ys.push(y);
        self.dxs.push(dx);
        self.dys.push(dy);
    }

    fn calc_player_min_max(&mut self) {
        if self.xs.len() > 0 {
            self.xs.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
            self.ys.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
            self.dxs.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
            self.dys.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

            self.min_x = self.xs[0];
            self.max_x = self.xs[self.xs.len() - 1];
            self.min_y = self.xs[0];
            self.max_y = self.xs[self.ys.len() - 1];

            self.max_abs_dx = self.dxs[self.dxs.len() - 1];
            self.max_abs_dy = self.dys[self.dys.len() - 1];
        }
    }

    fn add_arena_buffer(&mut self, arena_width: f32, arena_height: f32) {
        //this is the extra buffer space that the camera gives
        //offset was 80.0 for combat and 160.0 for race mode in old rally_game project
        if self.xs.len() > 0 {
            let dx_offset = 80.0 + 1.0 * self.max_abs_dx;
            let dy_offset = 80.0 + 1.0 * self.max_abs_dy;

            self.min_x = (self.min_x - dx_offset).max(0.0);
            self.max_x = (self.max_x + dx_offset).min(arena_width);
            self.min_y = (self.min_y - dy_offset).max(-40.0);
            self.max_y = (self.max_y + dy_offset).min(arena_height);
        }
    }

    pub fn calc_bounds_on_players_and_arena(&mut self, arena_width: f32, arena_height: f32) {
        self.calc_player_min_max();

        self.add_arena_buffer(
            arena_width,
            arena_height
        );
    }

    pub fn get_target_x(&self) -> f32 {
        self.min_x + (self.max_x - self.min_x) / 2.0
    }
    pub fn get_target_y(&self) -> f32 {
        self.min_x + (self.max_x - self.min_x) / 2.0
    }
    pub fn get_span_x(&self) -> f32 {
        self.max_x - self.min_x
    }
    pub fn get_span_y(&self) -> f32 {
        self.max_y - self.min_y
    }
}