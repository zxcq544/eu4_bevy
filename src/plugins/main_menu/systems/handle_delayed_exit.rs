use crate::plugins::main_menu::resources::exit_delay_timer::ExitDelayTimer;
use bevy::prelude::*;

pub fn handle_delayed_exit(
    time: Res<Time>,
    mut exit_timer: ResMut<ExitDelayTimer>,
    mut app_exit_writer: MessageWriter<AppExit>,
) {
    if exit_timer.should_exit {
        exit_timer.timer.tick(time.delta());
        if exit_timer.timer.just_finished() {
            info!("Delayed exit");
            app_exit_writer.write(AppExit::Success);
        }
    }
}
