use crate::{
    plugins::initial_boot_step::resources::initial_boot_step_timer::InitialBootStepTimer,
    shared::settings::settings::Settings,
};
use bevy::prelude::*;

pub fn start_initial_timer(mut commands: Commands, settings: Res<Settings>) {
    let timer = InitialBootStepTimer::new(settings.initial_bootscreen_show_time);
    commands.insert_resource(timer);
}
