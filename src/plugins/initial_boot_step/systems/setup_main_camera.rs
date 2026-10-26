use crate::shared::main_camera::main_camera::MainCameraEntity;
use bevy::prelude::*;

pub fn setup_main_camera(mut commands: Commands) {
    // commands.spawn((Camera3d::default(), MainCameraEntity));
    // let map_center = Vec3::new(2816.0, 0.0, 1024.0); // Для карты 5632x2048

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(100.0, 0.0, 3000.0) // Камера стоит сбоку и высоко
            .looking_at(
                Vec3 {
                    x: 1000.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vec3::Y,
            ),
        MainCameraEntity,
    ));
}

// use bevy::prelude::*;

// #[derive(Component)]
// pub struct MainCameraEntity;
// TODO: uncomment when actual data about cam position is available
// pub fn setup_main_camera(mut commands: Commands) {
//     // Для дефолтной карты EU4 (5632x2048) центр будет примерно в x: 2816, z: 1024
//     let camera_x = 2816.0 / 2.0; // Поставим камеру для начала ближе к левому краю
//     let camera_z = 1024.0 / 2.0;
//     let camera_y = 1500.0; // Поднимаем высоко в небо, чтобы увидеть ландшафт

//     commands.spawn((
//         Camera3d::default(),
//         Transform::from_xyz(camera_x, camera_y, camera_z)
//             // Заставляем камеру смотреть строго вниз на плоскость карты (X, Z)
//             .looking_at(Vec3::new(camera_x, 0.0, camera_z), Vec3::NEG_Z),
//         MainCameraEntity,
//     ));
// }
