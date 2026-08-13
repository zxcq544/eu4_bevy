use bevy::prelude::*;

use crate::{
    core::states::GameState,
    plugins::initial_boot_step::resources::initial_boot_at_least_timer::InitialBootAtLeastTimeout,
};

pub fn check_if_time_elapsed(
    time: Res<Time>,
    mut timeout: ResMut<InitialBootAtLeastTimeout>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    timeout.timer.tick(time.delta());
    if timeout.timer.just_finished() {
        info!("Timeout elapsed");
        next_state.set(GameState::LoadingAssets);
    }
}
