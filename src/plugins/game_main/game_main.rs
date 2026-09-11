// use crate::core::states::GameState;
// use bevy::prelude::*;
// pub struct GameMainPlugin;

// impl Plugin for GameMainPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_systems(OnEnter(GameState::GameMain), setup_game_main);
//     }
// }

// fn setup_game_main(mut commands: Commands, asset_server: Res<AssetServer>) {
//     commands.spawn((
//         Camera2dBundle {
//             camera: Camera {
//                 priority: 0,
//                 ..default()
//             },
//             ..default()
//         },
//         Transform2::from_xy(0.0, 0.0),
//         GlobalTransform::default(),
//         Visibility::default(),
//         ComputedVisibility::default(),
//         Name::new("Camera"),
//         // Camera2d,
//         // Camera,
//         // Transform,
//         // GlobalTransform,
//         // Visibility,
//         // ComputedVisibility,
//     ));
//     commands.spawn((
//         CameraUiCameraBundle::default(),
//         Transform2::from_xy(0.0, 0.0),
//         GlobalTransform::default(),
//         Visibility::default(),
//         ComputedVisibility::default(),
//         Name::new("Camera UI"),
//     ));
//     commands.spawn((
//         Transform2::from_xy(0.0, 0.0),
//         GlobalTransform::default(),
//         Visibility::default(),
//         ComputedVisibility::default(),
//         Name::new("Camera 2D"),
//     ));
// }
