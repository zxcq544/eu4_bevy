use crate::{
    core::states::GameState,
    plugins::loading_assets::resources::timer_for_main_loading_step::TimerForMainLoadingStep,
};
use bevy::prelude::*;

pub fn whole_setup_step_for_main_loading(
    mut timer: ResMut<TimerForMainLoadingStep>,
    time: Res<Time>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    timer.timer.tick(time.delta());
    if timer.timer.just_finished() {
        info!("current state is {:?}", current_state.get());
        info!("moving to main menu state");
        next_state.set(GameState::MainMenu);
    }
}
