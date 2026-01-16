use super::HEIGHT;
use super::OnLevel;
use super::WIDTH;
use crate::GlobalPlayer;
use crate::levels::GameState;
use crate::levels::beer_chasing::LevelState;
use bevy::prelude::*;

const MS: f32 = 200.0;

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin {
    pub game_state: GameState,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(self.game_state), setup_player)
            .add_systems(
                Update,
                move_player.run_if(
                    in_state(self.game_state).and(resource_exists_and_equals(LevelState::Running)),
                ),
            );
    }
}
pub fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(0.25, 0.25, 0.0)),
        OnLevel,
        Player,
        GlobalPlayer,
        Sprite {
            image: asset_server.load("beer.png"),
            ..default()
        },
    ));
}

fn move_player(
    key: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut transform: Single<&mut Transform, With<Player>>,
) {
    if key.pressed(KeyCode::ArrowLeft) {
        transform.translation.x =
            (transform.translation.x - MS * time.delta_secs()).max(-(WIDTH as f32) / 2.0);
    }
    if key.pressed(KeyCode::ArrowRight) {
        transform.translation.x =
            (transform.translation.x + MS * time.delta_secs()).min(WIDTH as f32 / 2.0);
    }
    if key.pressed(KeyCode::ArrowUp) {
        transform.translation.y =
            (transform.translation.y + MS * time.delta_secs()).min(HEIGHT as f32 / 2.0);
    }
    if key.pressed(KeyCode::ArrowDown) {
        transform.translation.y =
            (transform.translation.y - MS * time.delta_secs()).max(-(WIDTH as f32) / 2.0);
    }
}
