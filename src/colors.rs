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
            Self::Player => Color::Srgba(Srgba::new(1.0, 0.0, 0.0, 1.0)),
            Self::GroundLeft => Color::Srgba(Srgba::new(0.0, 1.0, 0.5, 1.0)),
            Self::GroundRight => Color::Srgba(Srgba::new(0.5, 1.0, 0.0, 1.0)),
            Self::GroundCenter => Color::Srgba(Srgba::new(0.0, 1.0, 0.0, 1.0)),
        }
    }
}
