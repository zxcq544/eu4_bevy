use crate::plugins::main_camera::{
    components::main_camera_entity::MainCameraEntity, resources::camera_config::CameraConfig,
};
use bevy::{
    input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel},
    prelude::*,
};

pub fn camera_control_system(
    buttons: Res<ButtonInput<MouseButton>>,
    mut mouse_motion_events: MessageReader<MouseMotion>,
    mut mouse_wheel_events: MessageReader<MouseWheel>,
    config: Res<CameraConfig>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<MainCameraEntity>>,
) {
    let mut camera_transform = match query.single_mut() {
        Ok(t) => t,
        Err(_) => return,
    };

    // 1. Handle Map Panning (Middle Click Drag)
    if buttons.pressed(MouseButton::Middle) {
        let mut delta = Vec2::ZERO;
        for event in mouse_motion_events.read() {
            delta += event.delta;
        }

        if delta != Vec2::ZERO {
            // Scale movement based on current height so dragging feels consistent when zoomed out
            let height_factor = camera_transform.translation.y / config.max_height;
            let pan_multiplier = config.pan_speed * height_factor * time.delta_secs();

            // Moving mouse up/down moves camera along Z axis, left/right moves along X axis
            camera_transform.translation.x -= delta.x * pan_multiplier;
            camera_transform.translation.z -= delta.y * pan_multiplier;
        }
    } else {
        // Clear events if not pressing middle click to avoid drift later
        mouse_motion_events.clear();
    }

    // 2. Handle Zoom & Dynamic Pitch (Mouse Wheel)
    let mut scroll_delta = 0.0;
    for event in mouse_wheel_events.read() {
        scroll_delta += match event.unit {
            MouseScrollUnit::Line => event.y,
            MouseScrollUnit::Pixel => event.y * 0.1,
        };
    }

    if scroll_delta != 0.0 {
        // Calculate new height (Y position)
        let mut new_y = camera_transform.translation.y - (scroll_delta * config.zoom_speed);
        new_y = new_y.clamp(config.min_height, config.max_height);
        camera_transform.translation.y = new_y;

        // Calculate interpolation factor (0.0 = fully zoomed out, 1.0 = fully zoomed in)
        // Inverse lerp: maps height from [min, max] to [1.0, 0.0]
        let zoom_t = (config.max_height - new_y) / (config.max_height - config.min_height);

        // Interpolate pitch between min_pitch (straight down) and max_pitch (angled)
        let current_pitch = config.min_pitch + zoom_t * (config.max_pitch - config.min_pitch);

        // Apply rotation: Base rotation is looking down (Quat::from_rotation_x(pitch))
        // We use NEG_Z or appropriate up vectors depending on how your coordinates track.
        // This resets rotation cleanly without gimbal-locking or accumulative drifting:
        camera_transform.rotation = Quat::from_rotation_x(current_pitch);
    }
}

// WITH LIMITS
// use bevy::{input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel}, prelude::*};

// #[derive(Component)]
// pub struct MainCameraEntity;

// #[derive(Resource)]
// pub struct CameraConfig {
//     pub pan_speed: f32,
//     pub zoom_speed: f32,
//     pub min_height: f32,
//     pub max_height: f32,
//     pub min_pitch: f32,
//     pub max_pitch: f32,
//     // Размеры вашей карты (например, для EU4: 5632.0 x 2048.0)
//     pub map_width: f32,
//     pub map_height: f32,
// }

// impl Default for CameraConfig {
//     fn default() -> Self {
//         Self {
//             pan_speed: 1.5,
//             zoom_speed: 150.0,
//             min_height: 200.0,
//             max_height: 2000.0,
//             min_pitch: -std::f32::consts::FRAC_PI_2, // -90 градусов
//             max_pitch: -std::f32::consts::FRAC_PI_4, // -45 градусов
//             map_width: 5632.0,
//             map_height: 2048.0,
//         }
//     }
// }

// pub fn camera_control_system(
//     buttons: Res<ButtonInput<MouseButton>>,
//     mut mouse_motion_events: MessageReader<MouseMotion>,
//     mut mouse_wheel_events: MessageReader<MouseWheel>,
//     config: Res<CameraConfig>,
//     time: Res<Time>,
//     mut query: Query<&mut Transform, With<MainCameraEntity>>,
// ) {
//     let mut camera_transform = match query.get_single_mut() {
//         Ok(t) => t,
//         Err(_) => return,
//     };

//     // --- 1. Обработка Зума и Наклона (Mouse Wheel) ---
//     let mut scroll_delta = 0.0;
//     for event in mouse_wheel_events.read() {
//         scroll_delta += match event.unit {
//             MouseScrollUnit::Line => event.y,
//             MouseScrollUnit::Pixel => event.y * 0.1,
//         };
//     }

//     let mut current_pitch = config.min_pitch;

//     if scroll_delta != 0.0 {
//         let mut new_y = camera_transform.translation.y - (scroll_delta * config.zoom_speed);
//         new_y = new_y.clamp(config.min_height, config.max_height);
//         camera_transform.translation.y = new_y;

//         let zoom_t = (config.max_height - new_y) / (config.max_height - config.min_height);
//         current_pitch = config.min_pitch + zoom_t * (config.max_pitch - config.min_pitch);

//         camera_transform.rotation = Quat::from_rotation_x(current_pitch);
//     } else {
//         // Если колесико не крутили, высчитываем текущий pitch на основе текущей высоты y
//         let zoom_t = (config.max_height - camera_transform.translation.y) / (config.max_height - config.min_height);
//         current_pitch = config.min_pitch + zoom_t * (config.max_pitch - config.min_pitch);
//     }

//     // --- 2. Обработка Перемещения (Middle Click Drag) ---
//     if buttons.pressed(MouseButton::Middle) {
//         let mut delta = Vec2::ZERO;
//         for event in mouse_motion_events.read() {
//             delta += event.delta;
//         }

//         if delta != Vec2::ZERO {
//             let height_factor = camera_transform.translation.y / config.max_height;
//             let pan_multiplier = config.pan_speed * height_factor * time.delta_secs();

//             camera_transform.translation.x -= delta.x * pan_multiplier;
//             camera_transform.translation.z -= delta.y * pan_multiplier;
//         }
//     } else {
//         mouse_motion_events.clear();
//     }

//     // --- 3. Динамическое ограничение границ (Clamping) ---
//     // Вычисляем, куда именно смотрит центр камеры на плоскости земли (Y = 0) с учетом угла наклона.
//     // Когда угол -90 (прямо вниз), смещение forward_offset равно 0.
//     // Когда угол -45, камера смотрит вперед, и точка пересечения с землей смещается по оси Z.
//     let angle_from_horizontal = -current_pitch; // Переводим в положительный угол
//     let forward_offset = camera_transform.translation.y / angle_horizontal.tan();

//     // Ограничиваем точку, куда смотрит камера, рамками карты
//     // По оси X ограничение прямое (камера смотрит строго по центру)
//     let target_x = camera_transform.translation.x.clamp(0.0, config.map_width);
//     // По оси Z учитываем смещение взгляда вперед
//     let target_z = (camera_transform.translation.z - forward_offset).clamp(0.0, config.map_height);

//     // Возвращаем координаты самой камеры обратно на основе ограниченной целевой точки
//     camera_transform.translation.x = target_x;
//     camera_transform.translation.z = target_z + forward_offset;
// }

// WITH SMOOTH MOVEMENT
// use bevy::{input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel}, prelude::*};

// #[derive(Component)]
// pub struct MainCameraEntity;

// /// Компонент для хранения желаемой позиции и угла камеры, к которым мы плавно движемся
// #[derive(Component, Default)]
// pub struct CameraTarget {
//     pub position: Vec3,
//     pub pitch: f32,
// }

// #[derive(Resource)]
// pub struct CameraConfig {
//     pub pan_speed: f32,
//     pub zoom_speed: f32,
//     pub min_height: f32,
//     pub max_height: f32,
//     pub min_pitch: f32,
//     pub max_pitch: f32,
//     pub map_width: f32,
//     pub map_height: f32,
//     pub smoothness: f32, // Чем выше значение, тем быстрее и резче камера догоняет цель (например, 10.0)
// }

// impl Default for CameraConfig {
//     fn default() -> Self {
//         Self {
//             pan_speed: 1.5,
//             zoom_speed: 150.0,
//             min_height: 200.0,
//             max_height: 2000.0,
//             min_pitch: -std::f32::consts::FRAC_PI_2,
//             max_pitch: -std::f32::consts::FRAC_PI_4,
//             map_width: 5632.0,
//             map_height: 2048.0,
//             smoothness: 8.0,
//         }
//     }
// }

// pub fn camera_control_system(
//     buttons: Res<ButtonInput<MouseButton>>,
//     mut mouse_motion_events: MessageReader<MouseMotion>,
//     mut mouse_wheel_events: MessageReader<MouseWheel>,
//     config: Res<CameraConfig>,
//     time: Res<Time>,
//     // Запрашиваем и трансформ камеры, и нашу новую цель
//     mut query: Query<(&mut Transform, &mut CameraTarget), With<MainCameraEntity>>,
// ) {
//     let (mut camera_transform, mut target) = match query.get_single_mut() {
//         Ok(res) => res,
//         Err(_) => return,
//     };

//     // Инициализация при первом запуске: если цель равна нулю, синхронизируем её с текущей позицией камеры
//     if target.position == Vec3::ZERO {
//         target.position = camera_transform.translation;
//         target.pitch = config.min_pitch;
//     }

//     // --- 1. Обработка Зума и Наклона для ЦЕЛИ ---
//     let mut scroll_delta = 0.0;
//     for event in mouse_wheel_events.read() {
//         scroll_delta += match event.unit {
//             MouseScrollUnit::Line => event.y,
//             MouseScrollUnit::Pixel => event.y * 0.1,
//         };
//     }

//     if scroll_delta != 0.0 {
//         let mut new_y = target.position.y - (scroll_delta * config.zoom_speed);
//         new_y = new_y.clamp(config.min_height, config.max_height);
//         target.position.y = new_y;

//         let zoom_t = (config.max_height - new_y) / (config.max_height - config.min_height);
//         target.pitch = config.min_pitch + zoom_t * (config.max_pitch - config.min_pitch);
//     }

//     // --- 2. Обработка Перемещения для ЦЕЛИ ---
//     if buttons.pressed(MouseButton::Middle) {
//         let mut delta = Vec2::ZERO;
//         for event in mouse_motion_events.read() {
//             delta += event.delta;
//         }

//         if delta != Vec2::ZERO {
//             // Используем текущую высоту ЦЕЛИ для расчета скорости переноса
//             let height_factor = target.position.y / config.max_height;
//             let pan_multiplier = config.pan_speed * height_factor * time.delta_secs();

//             target.position.x -= delta.x * pan_multiplier;
//             target.position.z -= delta.y * pan_multiplier;
//         }
//     } else {
//         mouse_motion_events.clear();
//     }

//     // --- 3. Ограничение границ ЦЕЛИ ---
//     let angle_from_horizontal = -target.pitch;
//     let forward_offset = target.position.y / angle_from_horizontal.tan();

//     let target_x = target.position.x.clamp(0.0, config.map_width);
//     let target_z = (target.position.z - forward_offset).clamp(0.0, config.map_height);

//     target.position.x = target_x;
//     target.position.z = target_z + forward_offset;

//     // --- 4. Плавная интерполяция (Lerp) камеры к ЦЕЛИ ---
//     // Формула сглаживания, не зависящая от частоты кадров (Frame-rate independent lerp)
//     let lerp_factor = (1.0 - (-config.smoothness * time.delta_secs()).exp()).clamp(0.0, 1.0);

//     // Плавное движение позиции
//     camera_transform.translation = camera_transform.translation.lerp(target.position, lerp_factor);

//     // Плавный поворот угла наклона
//     let current_pitch = camera_transform.rotation.to_euler(EulerRot::XYZ).0;
//     let next_pitch = current_pitch + (target.pitch - current_pitch) * lerp_factor;
//     camera_transform.rotation = Quat::from_rotation_x(next_pitch);
// }

// WITH BUTTONS
// use bevy::{input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel}, prelude::*};

// #[derive(Component)]
// pub struct MainCameraEntity;

// /// Компонент для хранения желаемой позиции и угла камеры, к которым мы плавно движемся
// #[derive(Component, Default)]
// pub struct CameraTarget {
//     pub position: Vec3,
//     pub pitch: f32,
// }

// #[derive(Resource)]
// pub struct CameraConfig {
//     pub pan_speed: f32,
//     pub keyboard_speed: f32, // Скорость перемещения клавишами (ед/сек)
//     pub zoom_speed: f32,
//     pub min_height: f32,
//     pub max_height: f32,
//     pub min_pitch: f32,
//     pub max_pitch: f32,
//     pub map_width: f32,
//     pub map_height: f32,
//     pub smoothness: f32,     // Коэффициент плавности догона цели
// }

// impl Default for CameraConfig {
//     fn default() -> Self {
//         Self {
//             pan_speed: 1.5,
//             keyboard_speed: 800.0, // Оптимальная скорость для клавиатуры
//             zoom_speed: 150.0,
//             min_height: 200.0,
//             max_height: 2000.0,
//             min_pitch: -std::f32::consts::FRAC_PI_2,
//             max_pitch: -std::f32::consts::FRAC_PI_4,
//             map_width: 5632.0,
//             map_height: 2048.0,
//             smoothness: 8.0,
//         }
//     }
// }

// pub fn camera_control_system(
//     mouse_buttons: Res<ButtonInput<MouseButton>>,
//     keyboard_buttons: Res<ButtonInput<KeyCode>>,
//     mut mouse_motion_events: MessageReader<MouseMotion>,
//     mut mouse_wheel_events: MessageReader<MouseWheel>,
//     config: Res<CameraConfig>,
//     time: Res<Time>,
//     mut query: Query<(&mut Transform, &mut CameraTarget), With<MainCameraEntity>>,
// ) {
//     let (mut camera_transform, mut target) = match query.get_single_mut() {
//         Ok(res) => res,
//         Err(_) => return,
//     };

//     // Первичная инициализация цели при старте
//     if target.position == Vec3::ZERO {
//         target.position = camera_transform.translation;
//         target.pitch = config.min_pitch;
//     }

//     // --- 1. Обработка Зума и Наклона ---
//     let mut scroll_delta = 0.0;
//     for event in mouse_wheel_events.read() {
//         scroll_delta += match event.unit {
//             MouseScrollUnit::Line => event.y,
//             MouseScrollUnit::Pixel => event.y * 0.1,
//         };
//     }

//     if scroll_delta != 0.0 {
//         let mut new_y = target.position.y - (scroll_delta * config.zoom_speed);
//         new_y = new_y.clamp(config.min_height, config.max_height);
//         target.position.y = new_y;

//         let zoom_t = (config.max_height - new_y) / (config.max_height - config.min_height);
//         target.pitch = config.min_pitch + zoom_t * (config.max_pitch - config.min_pitch);
//     }

//     // Рассчитываем множитель высоты, чтобы скорость перемещения (и мыши, и клавиатуры)
//     // ощущалась одинаково комфортно как вблизи, так и на максимальном удалении
//     let height_factor = target.position.y / config.max_height;

//     // --- 2. Перемещение мыши (Middle Click Drag) ---
//     if mouse_buttons.pressed(MouseButton::Middle) {
//         let mut delta = Vec2::ZERO;
//         for event in mouse_motion_events.read() {
//             delta += event.delta;
//         }

//         if delta != Vec2::ZERO {
//             let pan_multiplier = config.pan_speed * height_factor * time.delta_secs();
//             target.position.x -= delta.x * pan_multiplier;
//             target.position.z -= delta.y * pan_multiplier;
//         }
//     } else {
//         mouse_motion_events.clear();
//     }

//     // --- 3. Перемещение клавиатурой (WASD / Стрелочки) ---
//     let mut keyboard_direction = Vec3::ZERO;

//     if keyboard_buttons.pressed(KeyCode::KeyW) || keyboard_buttons.pressed(KeyCode::ArrowUp) {
//         keyboard_direction.z -= 1.0;
//     }
//     if keyboard_buttons.pressed(KeyCode::KeyS) || keyboard_buttons.pressed(KeyCode::ArrowDown) {
//         keyboard_direction.z += 1.0;
//     }
//     if keyboard_buttons.pressed(KeyCode::KeyA) || keyboard_buttons.pressed(KeyCode::ArrowLeft) {
//         keyboard_direction.x -= 1.0;
//     }
//     if keyboard_buttons.pressed(KeyCode::KeyD) || keyboard_buttons.pressed(KeyCode::ArrowRight) {
//         keyboard_direction.x += 1.0;
//     }

//     if keyboard_direction != Vec3::ZERO {
//         // Нормализуем вектор, чтобы движение по диагонали не было быстрее, чем по прямой
//         let move_speed = config.keyboard_speed * height_factor * time.delta_secs();
//         target.position += keyboard_direction.normalize() * move_speed;
//     }

//     // --- 4. Ограничение границ ЦЕЛИ ---
//     let angle_from_horizontal = -target.pitch;
//     let forward_offset = target.position.y / angle_from_horizontal.tan();

//     let target_x = target.position.x.clamp(0.0, config.map_width);
//     let target_z = (target.position.z - forward_offset).clamp(0.0, config.map_height);

//     target.position.x = target_x;
//     target.position.z = target_z + forward_offset;

//     // --- 5. Плавное движение (Lerp) камеры к ЦЕЛИ ---
//     let lerp_factor = (1.0 - (-config.smoothness * time.delta_secs()).exp()).clamp(0.0, 1.0);

//     // Интерполяция позиции
//     camera_transform.translation = camera_transform.translation.lerp(target.position, lerp_factor);

//     // Интерполяция угла наклона
//     let current_pitch = camera_transform.rotation.to_euler(EulerRot::XYZ).0;
//     let next_pitch = current_pitch + (target.pitch - current_pitch) * lerp_factor;
//     camera_transform.rotation = Quat::from_rotation_x(next_pitch);
// }
