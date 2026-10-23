use bevy::prelude::*;
use bevy::{
    ecs::system::{Query, Res},
};
use crate::core::states::OptionsTabState;
use crate::plugins::options::systems::options_top_buttons_sytems::options_top_buttons_system_united::OptionsTopTabButtonAction;

pub fn tab_highlight_system(
    current_tab: Res<State<OptionsTabState>>,
    mut buttons_query: Query<(&OptionsTopTabButtonAction, &mut Outline), With<Button>>,
) {
    if !current_tab.is_changed() {
        return;
    }
    for (action, mut outline) in &mut buttons_query {
        // Convert button action to tab state
        let is_active = match (action, *current_tab.get()) {
            (OptionsTopTabButtonAction::Video, OptionsTabState::Video) => true,
            (OptionsTopTabButtonAction::Audio, OptionsTabState::Audio) => true,
            (OptionsTopTabButtonAction::Game, OptionsTabState::Game) => true,
            (OptionsTopTabButtonAction::Controls, OptionsTabState::Controls) => true,
            _ => false,
        };

        if is_active {
            // Set active tab color
            outline.color = Color::srgb(1.0, 0.2, 0.5);
            outline.width = Val::Px(3.0);
        } else {
            // Set inactive tab color
            outline.color = Color::srgb(0.3, 0.3, 0.3);
            outline.width = Val::Px(1.0);
        }
    }
}
