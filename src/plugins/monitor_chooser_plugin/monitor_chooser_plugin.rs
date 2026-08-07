use crate::plugins::monitor_chooser_plugin::systems::*;
use bevy::prelude::*;

pub struct MonitorChooserPlugin;

impl Plugin for MonitorChooserPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                print_monitors.before(setup_window_monitor),
                setup_window_monitor,
            ),
        );
    }
}
