use crate::plugins::initial_boot_step::resources::initial_boot_step_timer::InitialBootStepTimer;
use bevy::prelude::*;
use settings::Settings;

pub fn start_initial_timer(mut commands: Commands, settings: Res<Settings>) {
    let timer = InitialBootStepTimer::new(settings.initial_bootscreen_show_time);
    commands.insert_resource(timer);
}
