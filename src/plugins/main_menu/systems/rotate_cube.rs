use crate::plugins::map::components::map_root_entity::MapRootEntity;
use bevy::prelude::*;

// pub fn rotate_cube_system(time: Res<Time>, mut query: Query<&mut Transform, With<RotatingCube>>) {
//     for mut transform in &mut query {
//         // Rotate around Y and X
//         // time.delta_secs() guarantees the same speed at any FPS
//         transform.rotate_y(1.0 * time.delta_secs());
//         transform.rotate_x(0.5 * time.delta_secs());
//     }
// }

// pub fn rotate_plane_system(time: Res<Time>, mut query: Query<&mut Transform, With<WaterPlane>>) {
//     for mut transform in &mut query {
//         // Rotate around Y and X
//         // time.delta_secs() гарантирует одинаковую скорость при любом FPS
//         transform.rotate_y(1.0 * time.delta_secs());
//         transform.rotate_x(0.5 * time.delta_secs());
//     }
// }

pub fn rotate_map_mesh_system(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<MapRootEntity>>,
) {
    for mut transform in &mut query {
        // Rotate around Y and X
        // time.delta_secs() guarantees the same speed at any FPS
        transform.rotate_y(1.0 * time.delta_secs());
        transform.rotate_x(0.5 * time.delta_secs());
    }
}
