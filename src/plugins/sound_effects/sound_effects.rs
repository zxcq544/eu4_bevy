// use crate::{
//     core::states::GameState,
//     plugins::sound_effects::resources::button_click_sound_effects::ButtonClickSoundEffects,
// };
// use bevy::prelude::*;

// #[derive(Component)]
// pub struct SoundEffectsPlugin;

// impl Plugin for SoundEffectsPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_systems(OnEnter(GameState::LoadingAssets), load_sound_effects);
//     }
// }

// // enum ButtonType {
// //     Ok,
// //     Back,
// //     General,
// // }

// fn load_sound_effects(asset_server: Res<AssetServer>, mut commands: Commands) {
//     let button_click_sound_effects = ButtonClickSoundEffects {
//         button_click_general: asset_server.load("sound/general_button_click.wav"),
//         button_click_ok: asset_server.load("sound/general_ok_button_click.wav"),
//         button_click_back: asset_server.load("sound/general_back_button_click.wav"),
//     };
//     commands.insert_resource(button_click_sound_effects);
//     info!("Sound effects loaded");
// }

// // fn play_button_click_sound_effect(
// //     button_click_sound_effects: Res<ButtonClickSoundEffects>,
// //     button_type: ButtonType,
// //     commands: &mut Commands,
// // ) {
// //     let required_sound = match button_type {
// //         ButtonType::Ok => button_click_sound_effects.button_click_ok.clone(),
// //         ButtonType::Back => button_click_sound_effects.button_click_back.clone(),
// //         ButtonType::General => button_click_sound_effects.button_click_general.clone(),
// //     };
// //     commands.spawn((AudioPlayer::new(required_sound), PlaybackSettings::DESPAWN));
// // }
