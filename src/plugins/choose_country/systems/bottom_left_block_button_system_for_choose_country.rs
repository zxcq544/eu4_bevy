use crate::{
    core::states::GameState,
    plugins::{
        choose_country::components::bottom_left_block_entity_for_choose_country::{
            BottomLeftBlockButtonActionsForChooseCountry, BottomLeftBlockButtonForChooseCountry,
        },
        sound_effects::{
            components::sound_effects_player::SoundEffectsPlayer,
            resources::button_click_sound_effects::ButtonClickSoundEffects,
        },
    },
    shared::settings::settings::Settings,
};
use bevy::{
    audio::Volume,
    input_focus::{FocusCause, InputFocus},
    prelude::*,
};

pub const NORMAL_BUTTON: Color = Color::srgb(1.0, 1.0, 1.0);
pub const HOVERED_BUTTON: Color = Color::srgb(1.15, 1.15, 1.15);
pub const PRESSED_BUTTON: Color = Color::srgb(0.85, 0.85, 0.85);

// Sets style of the button when mouse down
pub fn on_button_down(
    trigger: On<Pointer<Press>>,
    mut input_focus: ResMut<InputFocus>,
    mut button_query: Query<(&mut BottomLeftBlockButtonForChooseCountry, &mut ImageNode)>,
) {
    let entity = trigger.event_target();
    if let Ok((mut button, mut image_node)) = button_query.get_mut(entity) {
        input_focus.set(entity, FocusCause::Pressed);
        button.set_changed();
        image_node.color = PRESSED_BUTTON;
    }
}

// Observer for clicking on the button (Replaces Interaction::Pressed)
pub fn on_button_click(
    trigger: On<Pointer<Click>>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    sound_effects: Res<ButtonClickSoundEffects>,
    settings: Res<Settings>,
    mut input_focus: ResMut<InputFocus>,
    mut button_query: Query<(
        &mut BottomLeftBlockButtonForChooseCountry,
        &mut ImageNode,
        &BottomLeftBlockButtonActionsForChooseCountry,
    )>,
) {
    let entity = trigger.event_target();
    let Ok((mut button, mut image_node, action)) = button_query.get_mut(entity) else {
        return;
    };

    // Change the focus and change the visual
    input_focus.set(entity, FocusCause::Pressed);
    image_node.color = HOVERED_BUTTON;
    button.set_changed();

    // Logic of sound effects
    let required_sound = sound_effects.button_click_general.clone();
    commands.spawn((
        SoundEffectsPlayer,
        AudioPlayer::new(required_sound),
        PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Despawn,
            volume: Volume::Linear(
                settings
                    .volume_settings
                    .get_sfx_volume_multiplied_by_master(),
            ),
            ..default()
        },
    ));

    // Logic of actions (Change states)
    match action {
        BottomLeftBlockButtonActionsForChooseCountry::Options => {
            info!("Options button pressed");
            next_state.set(GameState::Options);
        }
        BottomLeftBlockButtonActionsForChooseCountry::Back => {
            info!("Back button pressed");
            next_state.set(GameState::MainMenu);
        }
    }
}

// Observer for hovering on the button (Replaces Interaction::Hovered)
pub fn on_button_hover(
    trigger: On<Pointer<Over>>,
    mut input_focus: ResMut<InputFocus>,
    mut button_query: Query<(&mut BottomLeftBlockButtonForChooseCountry, &mut ImageNode)>,
) {
    let entity = trigger.event_target();
    if let Ok((mut button, mut image_node)) = button_query.get_mut(entity) {
        input_focus.set(entity, FocusCause::Pressed);
        image_node.color = HOVERED_BUTTON;
        button.set_changed();
    }
}

// Observer for exiting the cursor beyond the button's limits (Replaces Interaction::None)
pub fn on_button_unhover(
    trigger: On<Pointer<Out>>,
    mut input_focus: ResMut<InputFocus>,
    mut button_query: Query<(&mut BottomLeftBlockButtonForChooseCountry, &mut ImageNode)>,
) {
    let entity = trigger.event_target();
    if let Ok((mut button, mut image_node)) = button_query.get_mut(entity) {
        input_focus.clear();
        image_node.color = NORMAL_BUTTON;
        button.set_changed();
    }
}
