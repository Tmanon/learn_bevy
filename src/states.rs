use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    InGame,
    LevelEditor,
    PlaceBodies,
    Paused,
}
