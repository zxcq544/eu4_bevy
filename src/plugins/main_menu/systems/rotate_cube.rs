use crate::plugins::main_menu::components::rotating_cube::RotatingCube;
use bevy::prelude::*;

pub fn rotate_cube_system(time: Res<Time>, mut query: Query<&mut Transform, With<RotatingCube>>) {
    for mut transform in &mut query {
        // Вращаем вокруг осей Y и X
        // time.delta_secs() гарантирует одинаковую скорость при любом FPS
        transform.rotate_y(1.0 * time.delta_secs());
        transform.rotate_x(0.5 * time.delta_secs());
    }
}
