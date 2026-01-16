use crate::GlobalPlayer;
use bevy::prelude::*;

pub struct UIPlugin;

#[derive(Component)]
struct PosText;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_text)
            .add_systems(Update, update_text);
    }
}

fn setup_text(mut commands: Commands) {
    commands.spawn((
        Text::new(""),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        PosText,
    ));
}

fn update_text(
    player: Single<&Transform, With<GlobalPlayer>>,
    mut text: Single<&mut Text, With<PosText>>,
) {
    let new_text = format!("({:.0},{:.0})", player.translation.x, player.translation.y);
    text.clear();
    text.push_str(&new_text);
}
