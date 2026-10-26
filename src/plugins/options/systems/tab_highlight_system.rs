use bevy::prelude::*;
use bevy::{
    ecs::system::{Query, Res},
};
use crate::core::states::OptionsTabState;
use crate::plugins::options::systems::options_top_buttons_sytems::options_top_buttons_system_united::OptionsTopTabButtonAction;

pub fn tab_highlight_system(
    current_tab: Res<State<OptionsTabState>>,
    mut buttons_query: Query<(&OptionsTopTabButtonAction, &mut ImageNode), With<Button>>,
) {
    if !current_tab.is_changed() {
        return;
    }
    for (action, mut image_node) in &mut buttons_query {
        // Compare assigned component of button action to tab state
        let is_active = match (action, *current_tab.get()) {
            (OptionsTopTabButtonAction::Video, OptionsTabState::Video) => true,
            (OptionsTopTabButtonAction::Audio, OptionsTabState::Audio) => true,
            (OptionsTopTabButtonAction::Game, OptionsTabState::Game) => true,
            (OptionsTopTabButtonAction::Controls, OptionsTabState::Controls) => true,
            _ => false,
        };

        if is_active {
            // Set active tab image rect
            image_node.rect = Some(Rect {
                min: Vec2::new(0.0, 0.0),
                max: Vec2::new(218.0, 47.0),
            });
        } else {
            // Set inactive tab image rect
            image_node.rect = Some(Rect {
                min: Vec2::new(218.0, 0.0),
                max: Vec2::new(444.0, 47.0),
            });
        }
    }
}
