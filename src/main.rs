mod colors;
mod levels;
mod ui;

use bevy::prelude::*;
use levels::LevelsPlugin;
use ui::UIPlugin;

#[derive(Component)]
struct GlobalPlayer;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .build()
                .disable::<bevy::audio::AudioPlugin>()
                .disable::<bevy::gilrs::GilrsPlugin>()
                .disable::<bevy::pbr::PbrPlugin>()
                .disable::<bevy::gltf::GltfPlugin>(),
        )
        .add_plugins(LevelsPlugin)
        .add_plugins(UIPlugin)
        .run();
}
