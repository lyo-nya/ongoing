use bevy::prelude::*;

#[derive(Clone, Copy)]
pub enum MyColors {
    Player,
    GroundLeft,
    GroundRight,
    GroundCenter,
}

impl MyColors {
    pub fn get(&self) -> Color {
        match self {
            Self::Player => Color::srgb(1.0, 0.0, 0.0),
            Self::GroundLeft => Color::srgb(0.0, 1.0, 0.5),
            Self::GroundRight => Color::srgb(0.5, 1.0, 0.0),
            Self::GroundCenter => Color::srgb(0.0, 1.0, 0.0),
        }
    }
}
