use bevy::prelude::*;

use crate::{
    core::states::GameState,
    plugins::monitor_chooser_plugin::systems::{
        print_monitors::print_monitors, setup_window_monitor::setup_window_monitor,
    },
};

pub struct MonitorChooserPlugin;

impl Plugin for MonitorChooserPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::MonitorAndWindowSetup),
            (
                print_monitors.before(setup_window_monitor),
                setup_window_monitor,
            ),
        );
    }
}
