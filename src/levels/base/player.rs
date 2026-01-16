use super::GameState;
use super::OnLevel;
use super::preset::{PLAYER_MS, PLAYER_SIZE, PLAYER_Y};
use crate::GlobalPlayer;
use crate::colors::MyColors;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin {
    pub game_state: GameState,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(self.game_state), setup_player)
            .add_systems(Update, move_player.run_if(in_state(self.game_state)));
    }
}

fn setup_player(mut commands: Commands) {
    commands.spawn((
        Sprite {
            color: MyColors::Player.get(),
            custom_size: Some(Vec2::new(PLAYER_SIZE, PLAYER_SIZE)),
            ..default()
        },
        Transform::from_xyz(0.0, PLAYER_Y, 1.0),
        Player,
        GlobalPlayer,
        OnLevel,
    ));
}

fn move_player(
    key: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut transform: Single<&mut Transform, With<Player>>,
) {
    if key.pressed(KeyCode::ArrowLeft) {
        transform.translation.x -= PLAYER_MS * time.delta_secs();
    }
    if key.pressed(KeyCode::ArrowRight) {
        transform.translation.x += PLAYER_MS * time.delta_secs();
    }
}
