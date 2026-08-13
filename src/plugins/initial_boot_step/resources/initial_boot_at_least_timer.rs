use bevy::prelude::*;
#[derive(Resource)]
pub struct InitialBootAtLeastTimeout {
    pub timer: Timer,
}

impl Default for InitialBootAtLeastTimeout {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(3.0, TimerMode::Once),
        }
    }
}
