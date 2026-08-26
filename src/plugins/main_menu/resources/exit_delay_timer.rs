use bevy::prelude::*;

#[derive(Resource)]
pub struct ExitDelayTimer {
    pub timer: Timer,
    pub should_exit: bool,
}
