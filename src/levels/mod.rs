use crate::{GlobalPlayer, levels::beer_chasing::LevelState};
use bevy::prelude::*;

mod base;
mod beer_chasing;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
enum GameState {
    #[default]
    Base,
    BeerChasing,
}

pub struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        GameState::setup(app);
    }
}

impl GameState {
    fn setup(app: &mut App) {
        app.init_state::<GameState>();
        base::init(app, GameState::Base);
        beer_chasing::init(app, GameState::BeerChasing);
        app.add_systems(Update, Self::switch_levels);
        app.add_systems(
            Update,
            Self::switch_gameover.run_if(
                in_state(GameState::BeerChasing)
                    .and(resource_exists_and_equals(LevelState::NeedsExit)),
            ),
        );
    }

    fn switch_gameover(mut next_state: ResMut<NextState<GameState>>) {
        next_state.set(GameState::Base)
    }
    fn switch_levels(
        player: Single<&Transform, With<GlobalPlayer>>,
        state: Res<State<GameState>>,
        mut next_state: ResMut<NextState<GameState>>,
    ) {
        match (state.get(), player.translation) {
            (GameState::Base, pos) if pos.x.floor() == 100.0 => {
                next_state.set(GameState::BeerChasing)
            }
            (GameState::BeerChasing, pos) if (pos - Vec3::new(0.0, 0.0, 0.0)).length() > 480.0 => {
                next_state.set(GameState::Base)
            }
            _ => {}
        }
    }
}

fn despawn_level<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn()
    }
}
