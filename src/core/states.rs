use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Boot,
    LoadingAssets,
    MainMenu,
    LoadingMap,
    Playing,
    Paused,
    GameOver,
}