use crate::plugins::loading_assets::resources::timer_for_main_loading_step::TimerForMainLoadingStep;
use bevy::prelude::*;
use settings::Settings;

pub fn start_timer_for_main_loading_step(mut commands: Commands, settings: Res<Settings>) {
    let timer = TimerForMainLoadingStep::new(settings.main_loading_screen_show_time);
    commands.insert_resource(timer);
}
