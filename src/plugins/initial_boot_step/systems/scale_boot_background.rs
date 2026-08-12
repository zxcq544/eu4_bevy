// use crate::{BootBackground, InitialBootingBackgroundScreen};
// use bevy::prelude::*;
// use bevy::window::Window;

// pub fn scale_boot_background(
//     window_query: Query<&Window>,
//     asset_server: Res<AssetServer>,
//     assets: Res<Assets<Image>>,
//     mut sprite_query: Query<&InitialBootingBackgroundScreen, With<BootBackground>>,
// ) {
//     // Получаем размеры основного окна
//     let Ok(window) = window_query.single() else {
//         return;
//     };
//     let window_width = window.width();
//     let window_height = window.height();

//     for (texture_handle) in sprite_query.iter_mut() {
//         // Проверяем, загрузилась ли уже текстура в память, чтобы узнать её размер
//         if let Some(image) = assets.get(texture_handle) {
//             let image_size = image.size_f32();

//             // Вычисляем коэффициенты масштабирования
//             let scale_x = window_width / image_size.x;
//             let scale_y = window_height / image_size.y;

//             // Вариант А: Растянуть ровно по границам окна (картинка может исказиться)
//             transform.scale = Vec3::new(scale_x, scale_y, 1.0);

//             // Вариант Б (Альтернатива): Заполнить экран без искажения пропорций (просто раскомментируйте)
//             // let max_scale = scale_x.max(scale_y);
//             // transform.scale = Vec3::new(max_scale, max_scale, 1.0);
//         }
//     }
// }
