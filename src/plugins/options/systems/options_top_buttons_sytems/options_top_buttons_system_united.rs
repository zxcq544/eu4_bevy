use crate::{
    core::states::OptionsTabState,
    plugins::sound_effects::sound_effects::{ButtonSoundType, PlayButtonClickSoundEvent},
};
use bevy::{
    input_focus::{FocusCause, InputFocus},
    prelude::*,
};

#[derive(Component, Clone, Default, Reflect)]
pub struct OptionsTopTabButton;

#[derive(Component, Clone, Default)]
pub enum OptionsTopTabButtonAction {
    #[default]
    NoAction,
    Video,
    Audio,
    Game,
    Controls,
}

// const NORMAL_BUTTON: Color = Color::srgb(1.0, 1.0, 1.0);
// const HOVERED_BUTTON: Color = Color::srgb(1.15, 1.15, 1.15);
// const PRESSED_BUTTON: Color = Color::srgb(0.85, 0.85, 0.85);

pub fn on_options_top_tab_button_click(
    trigger: On<Pointer<Click>>,
    mut next_state: ResMut<NextState<OptionsTabState>>,
    mut input_focus: ResMut<InputFocus>,
    mut button_query: Query<(
        &mut OptionsTopTabButton,
        &mut Node,
        &OptionsTopTabButtonAction,
    )>,
) {
    let entity = trigger.event_target();

    let Ok((mut button, mut node, action)) = button_query.get_mut(entity) else {
        return;
    };

    input_focus.set(entity, FocusCause::Pressed);
    node.border = UiRect::all(Val::Px(20.0));
    button.set_changed();

    match action {
        OptionsTopTabButtonAction::Video => {
            info!("Video button pressed");
            next_state.set(OptionsTabState::Video);
        }
        OptionsTopTabButtonAction::Audio => {
            info!("Audio button pressed");
            next_state.set(OptionsTabState::Audio);
        }
        OptionsTopTabButtonAction::Game => {
            info!("Game button pressed");
            next_state.set(OptionsTabState::Game);
        }
        OptionsTopTabButtonAction::Controls => {
            info!("Controls button pressed");
            next_state.set(OptionsTabState::Controls);
        }
        OptionsTopTabButtonAction::NoAction => {}
    }
}

pub fn on_options_top_tab_button_hover(
    trigger: On<Pointer<Over>>,
    mut input_focus: ResMut<InputFocus>,
    mut button_query: Query<(&mut OptionsTopTabButton, &mut Node)>,
) {
    let entity = trigger.event_target();
    if let Ok((mut button, mut node)) = button_query.get_mut(entity) {
        input_focus.set(entity, FocusCause::Pressed);
        node.border = UiRect::all(Val::Px(10.0));
        button.set_changed();
    }
}

pub fn on_options_top_tab_button_unhover(
    _trigger: On<Pointer<Out>>,
    mut input_focus: ResMut<InputFocus>,
    mut button_query: Query<(&mut OptionsTopTabButton, &mut Node)>,
) {
    let entity = _trigger.event_target();
    if let Ok((mut button, mut node)) = button_query.get_mut(entity) {
        input_focus.clear();
        node.border = UiRect::all(Val::Px(0.0));
        button.set_changed();
    }
}

pub fn on_options_top_tab_button_down(
    trigger: On<Pointer<Press>>,
    mut input_focus: ResMut<InputFocus>,
    mut commands: Commands,
    mut button_query: Query<(&mut OptionsTopTabButton, &mut Node)>,
) {
    let entity = trigger.event_target();
    if let Ok((mut button, mut node)) = button_query.get_mut(entity) {
        input_focus.set(entity, FocusCause::Pressed);
        button.set_changed();
        node.border = UiRect::all(Val::Px(20.0));
        // Logic of sound effects
        commands.trigger(PlayButtonClickSoundEvent(ButtonSoundType::General));
    }
}
