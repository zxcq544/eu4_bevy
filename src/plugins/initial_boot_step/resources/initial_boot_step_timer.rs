use std::time::Duration;

use bevy::prelude::*;

#[derive(Resource)]
pub struct InitialBootStepTimer {
    pub timer: Timer,
}

impl Default for InitialBootStepTimer {
    fn default() -> Self {
        Self {
            timer: Timer::new(Duration::from_secs(2), TimerMode::Once),
        }
    }
}

impl InitialBootStepTimer {
    pub fn new(time: u64) -> Self {
        Self {
            timer: Timer::new(Duration::from_secs(time), TimerMode::Once),
        }
    }
}
