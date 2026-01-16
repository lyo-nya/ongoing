use crate::levels::GameState;
use crate::levels::beer_chasing::LevelState;
use crate::levels::beer_chasing::player::Player;
use bevy::time::common_conditions::on_timer;
use std::time::Duration;

use super::OnLevel;
use super::Velocity;
use super::{HEIGHT, WIDTH};
use bevy::prelude::*;
use rand::Rng;

pub struct EnemyPlugin {
    pub game_state: GameState,
}

#[derive(Component)]
pub struct Enemy;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_enemy.run_if(in_state(self.game_state)))
            .add_systems(
                Update,
                chase_player.run_if(
                    in_state(self.game_state).and(resource_exists_and_equals(LevelState::Finished)),
                ),
            )
            .add_systems(
                Update,
                spawn_enemies
                    .run_if(
                        in_state(self.game_state)
                            .and(resource_exists_and_equals(LevelState::Running)),
                    )
                    .run_if(on_timer(Duration::from_secs(2))),
            )
            .add_systems(
                Update,
                bounce_off_walls
                    .run_if(in_state(self.game_state))
                    .after(move_enemy),
            );
    }
}

fn get_enemy() -> (Transform, Velocity) {
    let mut rng = rand::rng();
    let x: f32 = rng.random_range((-(WIDTH as f32) / 2.0)..(WIDTH as f32 / 2.0));
    let y: f32 = rng.random_range((-(HEIGHT as f32) / 2.0)..(HEIGHT as f32 / 2.0));
    let x_v: f32 = rng.random_range(-20.0..20.0);
    let y_v: f32 = rng.random_range(-20.0..20.0);
    (
        Transform::from_xyz(x, y, 0.0).with_scale(Vec3::new(0.2, 0.2, 0.0)),
        Velocity(Vec3::new(x_v, y_v, 0.0)),
    )
}

pub fn spawn_enemies(mut commands: Commands, asset_server: Res<AssetServer>) {
    let (t, v) = get_enemy();
    commands.spawn((
        t,
        OnLevel,
        Enemy,
        v,
        Sprite {
            image: asset_server.load("gopnik.png"),
            ..default()
        },
    ));
}

fn move_enemy(time: Res<Time>, transform: Query<(&mut Transform, &Velocity), With<Enemy>>) {
    for (mut t, v) in transform {
        t.translation += v.0 * time.elapsed_secs() * time.delta_secs();
    }
}

fn chase_player(
    player: Single<&Transform, With<Player>>,
    enemy: Query<(&Transform, &mut Velocity), With<Enemy>>,
) {
    for (t, mut v) in enemy {
        v.0 = (player.translation - t.translation) / 15.0;
    }
}

fn bounce_off_walls(mut query: Query<(&Transform, &mut Velocity), With<Enemy>>) {
    let x_limit = WIDTH as f32 / 2.0;
    let y_limit = HEIGHT as f32 / 2.0;

    for (transform, mut velocity) in &mut query {
        let translation = transform.translation;

        if translation.x >= x_limit && velocity.0.x > 0.0 {
            velocity.0.x *= -1.0;
        } else if translation.x <= -x_limit && velocity.0.x < 0.0 {
            velocity.0.x *= -1.0;
        }

        if translation.y >= y_limit && velocity.0.y > 0.0 {
            velocity.0.y *= -1.0;
        } else if translation.y <= -y_limit && velocity.0.y < 0.0 {
            velocity.0.y *= -1.0;
        }
    }
}
