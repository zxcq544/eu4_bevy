use crate::{
    core::states::GameState,
    plugins::{
        main_menu::components::main_menu_entity::{MainMenuButton, MainMenuButtonAction},
        sound_effects::sound_effects::{ButtonSoundType, PlayButtonClickSound},
    },
};
use bevy::{
    input_focus::{FocusCause, InputFocus},
    prelude::*,
};

const NORMAL_BUTTON: Color = Color::srgb(1.0, 1.0, 1.0);
const HOVERED_BUTTON: Color = Color::srgb(1.25, 1.25, 1.25);
const PRESSED_BUTTON: Color = Color::srgb(0.75, 0.75, 0.75);

pub fn on_main_menu_button_down(
    trigger: On<Pointer<Press>>,
    mut input_focus: ResMut<InputFocus>,
    mut commands: Commands,
    mut button_query: Query<(&mut MainMenuButton, &mut ImageNode)>,
) {
    let entity = trigger.event_target();
    if let Ok((mut button, mut image_node)) = button_query.get_mut(entity) {
        input_focus.set(entity, FocusCause::Pressed);
        button.set_changed();
        image_node.color = PRESSED_BUTTON;
        commands.trigger(PlayButtonClickSound(ButtonSoundType::General));
    }
}

/// Observer for clicking on the button (Replaces Interaction::Pressed)
pub fn on_main_menu_button_click(
    trigger: On<Pointer<Click>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut input_focus: ResMut<InputFocus>,
    mut app_exit_writer: MessageWriter<AppExit>,
    mut button_query: Query<(&mut MainMenuButton, &mut ImageNode, &MainMenuButtonAction)>,
) {
    let entity = trigger.event_target();

    // Filter out buttons that are not in the main menu
    let Ok((mut button, mut image_node, action)) = button_query.get_mut(entity) else {
        return;
    };

    // 1. Change the focus and change the visual
    input_focus.set(entity, FocusCause::Pressed);
    image_node.color = HOVERED_BUTTON;
    button.set_changed();

    // 2. Action logic
    match action {
        MainMenuButtonAction::SinglePlayer => {
            info!("Single player button pressed");
            next_state.set(GameState::ChooseCountry);
        }
        MainMenuButtonAction::Options => {
            info!("Options button pressed");
            next_state.set(GameState::Options);
        }
        MainMenuButtonAction::Quit => {
            info!("Quit button pressed");
            app_exit_writer.write(AppExit::Success);
        }
        _ => {}
    }
}

/// Observer for hovering on the button (Replaces Interaction::Hovered)
pub fn on_main_menu_button_hover(
    trigger: On<Pointer<Over>>,
    mut input_focus: ResMut<InputFocus>,
    mut button_query: Query<(&mut MainMenuButton, &mut ImageNode)>,
) {
    let entity = trigger.event_target();
    if let Ok((mut button, mut image_node)) = button_query.get_mut(entity) {
        input_focus.set(entity, FocusCause::Pressed);
        image_node.color = HOVERED_BUTTON;
        button.set_changed();
    }
}

/// Observer for exiting the cursor beyond the button's limits (Replaces Interaction::None)
pub fn on_main_menu_button_unhover(
    trigger: On<Pointer<Out>>,
    mut input_focus: ResMut<InputFocus>,
    mut button_query: Query<(&mut MainMenuButton, &mut ImageNode)>,
) {
    if button_query.get(trigger.event_target()).is_ok() {
        let (mut button, mut image_node) = button_query.get_mut(trigger.event_target()).unwrap();
        input_focus.clear();
        image_node.color = NORMAL_BUTTON;
        button.set_changed();
    }
}
