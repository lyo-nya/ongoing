use bevy::{prelude::*, window::WindowResized};

use crate::{GROUND_LEVEL, PLAYER_SIZE};

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Config>();
        app.add_systems(Update, on_resize);
    }
}

#[derive(Resource)]
pub struct Config {
    pub ground_y: f32,
    pub player_y: f32,
    pub ground_height: f32,
    pub player_height: f32,
    pub ground_tile_width: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self::from_resolution(1080.0, 1920.0)
    }
}

impl Config {
    pub fn from_resolution(width: f32, height: f32) -> Self {
        let ground_height = height * GROUND_LEVEL;
        let ground_y = -height / 2.0 + ground_height / 2.0;
        let player_height = ground_height * PLAYER_SIZE;
        Self {
            ground_height,
            ground_y,
            player_height,
            player_y: ground_y + ground_height / 2.0 + player_height / 2.0,
            ground_tile_width: width,
        }
    }

    pub fn update(&mut self, width: f32, height: f32) {
        *self = Self::from_resolution(width, height);
    }
}

pub fn on_resize(mut event: MessageReader<WindowResized>, mut config: ResMut<Config>) {
    for w in event.read() {
        config.update(w.width, w.height);
    }
}
