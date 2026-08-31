// use crate::plugins::{
//     main_menu::components::main_menu_entity::{MainMenuButton, MainMenuButtonSoundType},
//     sound_effects::resources::button_click_sound_effects::ButtonClickSoundEffects,
// };
// use bevy::prelude::*;

// pub fn main_menu_button_sounds(
//     interaction_query: Query<
//         (&Interaction, &MainMenuButtonSoundType),
//         (Changed<Interaction>, With<MainMenuButton>),
//     >,
//     mut commands: Commands,
//     button_click_sound_effects: Res<ButtonClickSoundEffects>,
// ) {
//     for (interaction, button_sound_type) in &interaction_query {
//         if *interaction == Interaction::Pressed {
//             let required_sound = match button_sound_type {
//                 MainMenuButtonSoundType::Ok => button_click_sound_effects.button_click_ok.clone(),
//                 MainMenuButtonSoundType::Back => {
//                     button_click_sound_effects.button_click_back.clone()
//                 }
//                 MainMenuButtonSoundType::General => {
//                     button_click_sound_effects.button_click_general.clone()
//                 }
//             };
//             commands.spawn((AudioPlayer::new(required_sound), PlaybackSettings::DESPAWN));
//         }
//     }
// }
