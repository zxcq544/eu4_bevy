use bevy::prelude::*;
use std::time::Duration;

#[derive(Resource)]
pub struct TimerForMainLoadingStep {
    pub timer: Timer,
}

impl Default for TimerForMainLoadingStep {
    fn default() -> Self {
        Self {
            timer: Timer::new(Duration::from_secs(2), TimerMode::Once),
        }
    }
}

impl TimerForMainLoadingStep {
    pub fn new(time: f32) -> Self {
        Self {
            timer: Timer::from_seconds(time, TimerMode::Once),
        }
    }
}
