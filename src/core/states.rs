use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    MonitorAndWindowSetup,
    Boot,
    LoadingAssets,
    PreMainMenuSetup,
    MainMenu,
    Options,
    LoadingMap,
    Playing,
    Paused,
    GameOver,
}

#[derive(SubStates, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[source(GameState = GameState::Options)]
pub enum OptionsTab {
    #[default]
    Video,
    Audio,
    Game,
    Controls,
}
