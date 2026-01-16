use crate::levels::beer_chasing::enemy::Enemy;
use crate::levels::beer_chasing::player::{Player, PlayerPlugin};
use crate::levels::{GameState, despawn_level};
use bevy::prelude::*;
mod enemy;
mod player;
use rand::Rng;

use enemy::EnemyPlugin;

const WIDTH: u32 = 800;
const HEIGHT: u32 = WIDTH;

#[derive(Component)]
pub struct OnLevel;

#[derive(Component)]
pub struct Velocity(Vec3);

#[derive(Resource, Debug, Clone, Eq, PartialEq)]
pub enum LevelState {
    Running,
    Finished,
    NeedsExit,
}

pub fn init(app: &mut App, game_state: GameState) {
    app.add_systems(OnEnter(game_state), setup)
        .add_systems(OnEnter(game_state), change_resolution_system)
        .add_systems(OnExit(game_state), despawn_level::<OnLevel>)
        .add_systems(OnExit(game_state), setdown)
        // .add_systems(Update)
        .add_plugins(EnemyPlugin { game_state })
        .add_plugins(PlayerPlugin { game_state })
        .add_systems(Update, detect_collision.run_if(in_state(game_state)))
        .add_systems(
            Update,
            detect_exit
                .run_if(in_state(game_state).and(resource_exists_and_equals(LevelState::Finished))),
        )
        .add_systems(
            Update,
            jitter
                .run_if(in_state(game_state).and(resource_exists_and_equals(LevelState::Finished))),
        );
}

pub fn setdown(mut commands: Commands) {
    commands.remove_resource::<LevelState>();
}

fn change_resolution_system(mut window: Single<&mut Window>) {
    window.resolution.set(WIDTH as f32, HEIGHT as f32);
}

pub fn setup(mut commands: Commands) {
    commands.insert_resource(LevelState::Running);
    commands.spawn((Camera2d, OnLevel, Transform::from_xyz(0.0, 0.0, 0.0)));
}

pub fn jitter(mut camera: Single<&mut Transform, With<Camera2d>>) {
    let mut rng = rand::rng();
    let x: f32 = rng.random_range(-10.0..10.0);
    let y: f32 = rng.random_range(-10.0..10.0);
    camera.translation = Vec3::new(x, y, 0.0);
}

pub fn detect_collision(
    player: Single<&Transform, With<Player>>,
    enemies: Query<&Transform, With<Enemy>>,
    mut level_state: ResMut<LevelState>,
) {
    let p_pos = player.translation;
    if enemies
        .iter()
        .any(|e| (e.translation - p_pos).length() < 40.0)
    {
        *level_state = LevelState::Finished
    }
}

pub fn detect_exit(
    player: Single<&Transform, With<Player>>,
    enemies: Query<&Transform, With<Enemy>>,
    mut level_state: ResMut<LevelState>,
) {
    let p_pos = player.translation;
    if enemies
        .iter()
        .all(|e| (e.translation - p_pos).length() < 10.0)
    {
        *level_state = LevelState::NeedsExit
    }
}
