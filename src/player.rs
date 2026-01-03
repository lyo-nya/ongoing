use crate::colors::MyColors;
use crate::{PLAYER_MS, PLAYER_SIZE, PLAYER_Y};
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player);
        app.add_systems(Update, move_player);
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
    ));
}

fn move_player(
    key: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut transform: Query<&mut Transform, With<Player>>,
) {
    let mut transform = transform.single_mut().expect("Only one player");
    if key.pressed(KeyCode::ArrowLeft) {
        transform.translation.x -= PLAYER_MS * time.delta_secs();
    }
    if key.pressed(KeyCode::ArrowRight) {
        transform.translation.x += PLAYER_MS * time.delta_secs();
    }
}
