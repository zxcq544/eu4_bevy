use bevy::prelude::*;

use crate::{print_monitors, setup_window_monitor};

pub struct MonitorChooserPlugin;

impl Plugin for MonitorChooserPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreStartup,
            (
                print_monitors.before(setup_window_monitor),
                setup_window_monitor,
            ),
        );
    }
}
