use super::GameState;
use super::OnLevel;
use super::player::Player;
use super::preset::CAMERA_OFFSET;
use bevy::prelude::*;

pub struct CameraPlugin {
    pub game_state: GameState,
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(self.game_state), setup_camera)
            .add_systems(Update, camera_follow.run_if(in_state(self.game_state)));
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Transform::from_xyz(0.0, 0.0, 0.0), OnLevel));
}

fn camera_follow(
    time: Res<Time>,
    player_transform: Single<&Transform, (With<Player>, Without<Camera2d>)>,
    mut camera_transform: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    let direction = player_transform.translation + Vec3::new(0.0, CAMERA_OFFSET, 0.0)
        - camera_transform.translation;
    if direction.length() > 0.0 {
        camera_transform.translation += direction * 0.975 * time.delta_secs();
    }
}
