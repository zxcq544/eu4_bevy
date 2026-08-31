// use crate::plugins::main_menu::{
//     components::main_menu_entity::{MainMenuButton, MainMenuButtonAction},
//     resources::exit_delay_timer::ExitDelayTimer,
// };
// use bevy::prelude::*;
// use settings::Settings;

// pub fn main_menu_button_actions(
//     interaction_query: Query<
//         (&Interaction, &MainMenuButtonAction),
//         (Changed<Interaction>, With<MainMenuButton>),
//     >,
//     mut exit_timer: ResMut<ExitDelayTimer>,
//     settings: Res<Settings>,
// ) {
//     for (interaction, menu_button_action) in &interaction_query {
//         if *interaction == Interaction::Pressed {
//             match menu_button_action {
//                 MainMenuButtonAction::Quit => {
//                     let exit_time_setting = settings.exit_delay_time;
//                     info!("Quit button pressed");
//                     exit_timer.timer = Timer::from_seconds(exit_time_setting, TimerMode::Once);
//                     exit_timer.should_exit = true;
//                 }
//                 _ => {}
//             }
//         }
//     }
// }
