use super::player::Player;
use super::preset::GROUND_WIDTH;
use super::preset::{GROUND_HEIGHT, GROUND_TILES, GROUND_Y, TILE_WIDTH, WIDTH};
use crate::colors::MyColors;
use crate::levels::GameState;
use crate::levels::base::OnLevel;
use bevy::prelude::*;

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct GroundTile;

pub struct GroundPlugin {
    pub game_state: GameState,
}

impl Plugin for GroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(self.game_state), setup_ground)
            .add_systems(
                Update,
                recycle_ground_sorted.run_if(in_state(self.game_state)),
            );
    }
}

pub fn setup_ground(mut commands: Commands, _assets: Res<AssetServer>) {
    let shape = Some(Vec2::new(TILE_WIDTH, GROUND_HEIGHT));
    let colors = vec![
        MyColors::GroundLeft,
        MyColors::GroundCenter,
        MyColors::GroundRight,
    ];
    commands
        .spawn((
            Ground,
            OnLevel,
            Transform::from_xyz(-(WIDTH as f32), GROUND_Y, 0.0),
            Visibility::default(),
        ))
        .with_children(move |parent| {
            colors
                .repeat(GROUND_TILES / colors.len() + 1)
                .iter()
                .zip(0..GROUND_TILES)
                .for_each(|(color, n)| {
                    parent.spawn((
                        Sprite {
                            color: color.get(),
                            custom_size: shape,
                            ..default()
                        },
                        GroundTile,
                        Transform::from_xyz(0.0 + (n as f32) * TILE_WIDTH, 0.0, 0.0),
                    ));
                });
        });
}

fn recycle_ground_sorted(
    player: Single<&Transform, With<Player>>,
    mut query: Query<
        (Entity, &mut Transform, &GlobalTransform),
        (With<GroundTile>, Without<Player>),
    >,
) {
    let threshold = player.translation.x;

    let mut tiles: Vec<(Entity, f32)> = query
        .iter()
        .map(|(e, _, g)| (e, g.translation().x))
        .collect();
    tiles.sort_by(|a, b| {
        a.1.partial_cmp(&b.1)
            .expect("X values should be non-NaN floats")
    });

    match (tiles.first(), tiles.last()) {
        (Some(&(first_tile, first_x)), Some(&(last_tile, last_x))) => {
            if last_x < threshold + 3.0 * TILE_WIDTH {
                if let Ok((_, mut first_transform, _)) = query.get_mut(first_tile) {
                    first_transform.translation.x += GROUND_WIDTH;
                }
            }
            if first_x > threshold - 3.0 * TILE_WIDTH {
                if let Ok((_, mut last_transform, _)) = query.get_mut(last_tile) {
                    last_transform.translation.x -= GROUND_WIDTH;
                }
            }
        }
        _ => panic!("You should have at least two tiles, buddy!"),
    }
}
