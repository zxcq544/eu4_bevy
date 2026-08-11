use bevy::prelude::*;
use bevy::window::Monitor;
pub fn print_monitors(q_monitors: Query<Entity, With<Monitor>>) {
    let count = q_monitors.iter().count();
    info!("Found {} monitors", count);
    for (i, entity) in q_monitors.iter().enumerate() {
        info!("Monitor {}: {:?}", i, entity);
    }
}
