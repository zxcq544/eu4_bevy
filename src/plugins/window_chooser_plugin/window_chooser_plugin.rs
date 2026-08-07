use crate::plugins::window_chooser_plugin::systems::*;
use bevy::prelude::*;

pub struct WindowChooserPlugin;

impl Plugin for WindowChooserPlugin {
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
