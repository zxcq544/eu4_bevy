use crate::plugins::main_menu::components::main_menu_entity::MainMenuButton;
use bevy::{
    input_focus::{FocusCause, InputFocus},
    prelude::*,
};

pub const NORMAL_BUTTON: Color = Color::srgb(1.0, 1.0, 1.0);
pub const HOVERED_BUTTON: Color = Color::srgb(1.25, 1.25, 1.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.75, 0.75, 0.75);

pub fn main_menu_button_hover(
    mut input_focus: ResMut<InputFocus>,
    mut interaction_query: Query<
        (Entity, &Interaction, &mut MainMenuButton, &mut ImageNode),
        Changed<Interaction>,
    >,
) {
    for (entity, interaction, mut button, mut image_node) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                input_focus.set(entity, FocusCause::Pressed);
                image_node.color = PRESSED_BUTTON;
                // The accessibility system's only update the button's state when the `Button` component is marked as changed.
                button.set_changed();
            }
            Interaction::Hovered => {
                input_focus.set(entity, FocusCause::Pressed);
                image_node.color = HOVERED_BUTTON;
                button.set_changed();
            }
            Interaction::None => {
                input_focus.clear();
                image_node.color = NORMAL_BUTTON;
                button.set_changed();
            }
        }
    }
}
