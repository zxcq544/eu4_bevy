use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    MonitorAndWindowSetup,
    Boot,
    LoadingAssets,
    MainMenu,
    LoadingMap,
    Playing,
    Paused,
    GameOver,
}

#[derive(SubStates, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[source(GameState = GameState::MainMenu)]
pub enum MainMenuStates {
    #[default]
    OptionsHidden,
    OnMainMenuOptionsScreen,
}
