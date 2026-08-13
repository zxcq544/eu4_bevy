use bevy::prelude::*;
use std::time::Duration;

use crate::{InitialBootAtLeastTimeout, states::GameState};

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
