use super::{GameState, despawn_level};
use bevy::prelude::*;
use camera::CameraPlugin;
use ground::GroundPlugin;
use player::PlayerPlugin;
use preset::{HEIGHT, WIDTH};

mod camera;
mod ground;
mod player;
mod preset;

#[derive(Component)]
pub struct OnLevel;

pub fn init(app: &mut App, game_state: GameState) {
    app.add_systems(OnExit(game_state), despawn_level::<OnLevel>)
        .add_systems(OnEnter(game_state), change_resolution_system)
        .add_plugins(CameraPlugin { game_state })
        .add_plugins(PlayerPlugin { game_state })
        .add_plugins(GroundPlugin { game_state });
}

fn change_resolution_system(mut window: Single<&mut Window>) {
    window.resolution.set(WIDTH as f32, HEIGHT as f32);
}
