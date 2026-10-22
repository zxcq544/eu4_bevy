use crate::{
    core::states::GameState,
    plugins::{
        options::components::options_ui_main_entity::{OptionsButton, OptionsButtonAction},
        sound_effects::sound_effects::{ButtonSoundType, PlayButtonClickSoundEvent},
    },
    shared::settings::settings::Settings,
};
use bevy::{
    input_focus::{FocusCause, InputFocus},
    prelude::*,
};
use std::{fs::File, path::Path};

const NORMAL_BUTTON: Color = Color::srgb(1.0, 1.0, 1.0);
const HOVERED_BUTTON: Color = Color::srgb(1.15, 1.15, 1.15);
const PRESSED_BUTTON: Color = Color::srgb(0.85, 0.85, 0.85);

fn save_settings_to_json(settings: &Settings) {
    let file_path = Path::new(&settings.file_name);
    info!("Saving settings to {:?}", file_path);
    if file_path.exists() {
        let mut file = File::create(file_path)
            .expect("file settings.json doesn't exist in root of game or blocked for write");
        serde_json::to_writer_pretty(&mut file, &settings).expect("Unable to write to file");
    } else {
        info!("File settings.json doesn't exist in root of game or blocked for write");
    }
}

pub fn on_options_bottom_button_click(
    trigger: On<Pointer<Click>>,
    mut next_state: ResMut<NextState<GameState>>,
    previous_state: Option<Res<PreviousState<GameState>>>,
    mut settings: ResMut<Settings>,
    mut input_focus: ResMut<InputFocus>,
    mut button_query: Query<(&mut OptionsButton, &mut ImageNode, &OptionsButtonAction)>,
) {
    let entity = trigger.event_target();

    let Ok((mut button, mut image_node, action)) = button_query.get_mut(entity) else {
        return;
    };

    input_focus.set(entity, FocusCause::Pressed);
    image_node.color = HOVERED_BUTTON;
    button.set_changed();

    match action {
        OptionsButtonAction::Apply => {
            info!("Apply button pressed");
            settings.initial_bootscreen_show_time = 2.0;
            save_settings_to_json(&settings);
        }
        OptionsButtonAction::Back => {
            info!("Back button pressed");
            if let Some(previous_game_state) = &previous_state {
                next_state.set(previous_game_state.get().clone());
            }
        }
        OptionsButtonAction::NoAction => {}
    }
}

pub fn on_options_bottom_button_hover(
    trigger: On<Pointer<Over>>,
    mut input_focus: ResMut<InputFocus>,
    mut button_query: Query<(&mut OptionsButton, &mut ImageNode)>,
) {
    let entity = trigger.event_target();
    if let Ok((mut button, mut image_node)) = button_query.get_mut(entity) {
        input_focus.set(entity, FocusCause::Pressed);
        image_node.color = HOVERED_BUTTON;
        button.set_changed();
    }
}

pub fn on_options_bottom_button_unhover(
    _trigger: On<Pointer<Out>>,
    mut input_focus: ResMut<InputFocus>,
    mut button_query: Query<(&mut OptionsButton, &mut ImageNode)>,
) {
    let entity = _trigger.event_target();
    if let Ok((mut button, mut image_node)) = button_query.get_mut(entity) {
        input_focus.clear();
        image_node.color = NORMAL_BUTTON;
        button.set_changed();
    }
}

pub fn on_options_bottom_button_down(
    trigger: On<Pointer<Press>>,
    mut input_focus: ResMut<InputFocus>,
    mut commands: Commands,
    mut button_query: Query<(&mut OptionsButton, &mut ImageNode)>,
) {
    let entity = trigger.event_target();
    if let Ok((mut button, mut image_node)) = button_query.get_mut(entity) {
        input_focus.set(entity, FocusCause::Pressed);
        button.set_changed();
        image_node.color = PRESSED_BUTTON;
        // Logic of sound effects
        commands.trigger(PlayButtonClickSoundEvent(ButtonSoundType::General));
    }
}
