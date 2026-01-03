mod camera;
mod colors;
mod ground;
mod player;

use bevy::prelude::*;
use camera::CameraPlugin;
use ground::GroundPlugin;
use player::PlayerPlugin;

const GROUND_LEVEL: f32 = 0.15;
const PLAER_SIZE_IN_GROUNDS: f32 = 0.5;
const PLAYER_MS_IN_SIZES: f32 = 8.0;
const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
const GROUND_HEIGHT: f32 = (HEIGHT as f32) * GROUND_LEVEL;
const PLAYER_SIZE: f32 = GROUND_HEIGHT * PLAER_SIZE_IN_GROUNDS;
const PLAYER_MS: f32 = PLAYER_MS_IN_SIZES * PLAYER_SIZE;
const GROUND_Y: f32 = -(HEIGHT as f32) / 2.0 + GROUND_HEIGHT / 2.0;
const PLAYER_Y: f32 = GROUND_Y + GROUND_HEIGHT / 2.0 + PLAYER_SIZE / 2.0;
const GROUND_TILES: usize = 10;
const GROUND_WIDTH: f32 = (WIDTH as f32) * 2.0;
const TILE_WIDTH: f32 = GROUND_WIDTH / (GROUND_TILES as f32);
const CAMERA_OFFSET: f32 = -PLAYER_Y;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (WIDTH, HEIGHT).into(),
                        ..default()
                    }),
                    ..default()
                })
                .build()
                .disable::<bevy::audio::AudioPlugin>()
                .disable::<bevy::gilrs::GilrsPlugin>()
                .disable::<bevy::pbr::PbrPlugin>()
                .disable::<bevy::gltf::GltfPlugin>(),
        )
        .add_plugins(GroundPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(CameraPlugin)
        .run();
}
