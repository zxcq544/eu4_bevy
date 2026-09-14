use crate::{
    plugins::loading_assets::resources::timer_for_main_loading_step::TimerForMainLoadingStep,
    shared::settings::settings::Settings,
};
use bevy::prelude::*;

pub fn start_timer_for_main_loading_step(mut commands: Commands, settings: Res<Settings>) {
    let timer = TimerForMainLoadingStep::new(settings.main_loading_screen_show_time);
    commands.insert_resource(timer);
}
