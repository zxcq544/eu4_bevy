use bevy::prelude::*;
use bevy::ui_widgets::{SetSliderValue, SliderValueChange};
use bevy::{
    input_focus::{FocusCause, InputFocus},
};
use crate::plugins::options::components::options_middle_block::audio_slider_widget::options_slider_base::OptionUiAudioSlider;
use crate::plugins::options::components::options_middle_block::audio_slider_widget::slider_right_arrow_button::OptionsUIAudioSliderButtonRight;
use crate::plugins::sound_effects::sound_effects::{ButtonSoundType, PlayButtonClickSoundEvent};
use crate::shared::audio_channel::audio_channel::AudioChannelUIMarker;
use crate::shared::settings::settings::Settings;

pub const NORMAL_BUTTON: Color = Color::srgb(1.0, 1.0, 1.0);
pub const HOVERED_BUTTON: Color = Color::srgb(1.15, 1.15, 1.15);
pub const PRESSED_BUTTON: Color = Color::srgb(0.85, 0.85, 0.85);

pub fn on_volume_increase_down(
    trigger: On<PointerPress>,
    mut input_focus: ResMut<InputFocus>,
    mut button_query: Query<(&mut OptionsUIAudioSliderButtonRight, &mut ImageNode)>,
) {
    let button_entity = trigger.event_target();
    if let Ok((mut button, mut image_node)) = button_query.get_mut(button_entity) {
        input_focus.clear();
        image_node.color = PRESSED_BUTTON;
        button.set_changed();
    }
}

/// Функция-наблюдатель для обработки клика (аналог Interaction::Pressed)
pub fn on_volume_increase_click(
    trigger: On<PointerClick>,
    mut commands: Commands,
    settings: Res<Settings>,
    mut input_focus: ResMut<InputFocus>,
    mut button_query: Query<(
        &mut OptionsUIAudioSliderButtonRight,
        &mut ImageNode,
        &AudioChannelUIMarker,
    )>,
    sliders_query: Query<(Entity, &AudioChannelUIMarker), With<OptionUiAudioSlider>>,
) {
    let button_entity = trigger.event_target();

    let Ok((mut button, mut image_node, audio_channel_of_button)) =
        button_query.get_mut(button_entity)
    else {
        return;
    };

    input_focus.set(button_entity, FocusCause::Pressed);
    image_node.color = HOVERED_BUTTON;
    button.set_changed();

    // Sound effect logic
    commands.trigger(PlayButtonClickSoundEvent(ButtonSoundType::General));

    // Find slider with same audio channel
    for (slider_entity, audio_channel_of_slider) in &sliders_query {
        if *audio_channel_of_slider == *audio_channel_of_button {
            let current_value = audio_channel_of_slider.get_volume_from_settings(&settings);
            let new_value = (current_value + 0.1).clamp(0.0, 1.0);

            commands.trigger(SetSliderValue {
                entity: slider_entity,
                change: SliderValueChange::Absolute(new_value),
            });

            info!(
                "Volume increased click {:?}. New value: {}",
                audio_channel_of_button, new_value
            );
            break;
        }
    }
}

pub fn on_volume_increase_over(
    trigger: On<PointerOver>,
    mut input_focus: ResMut<InputFocus>,
    mut button_query: Query<(&mut OptionsUIAudioSliderButtonRight, &mut ImageNode)>,
) {
    let button_entity = trigger.event_target();
    if let Ok((mut button, mut image_node)) = button_query.get_mut(button_entity) {
        input_focus.set(button_entity, FocusCause::Pressed);
        image_node.color = HOVERED_BUTTON;
        button.set_changed();
    }
}

pub fn on_volume_increase_out(
    trigger: On<PointerOut>,
    mut input_focus: ResMut<InputFocus>,
    mut button_query: Query<(&mut OptionsUIAudioSliderButtonRight, &mut ImageNode)>,
) {
    let button_entity = trigger.event_target();
    if let Ok((mut button, mut image_node)) = button_query.get_mut(button_entity) {
        input_focus.clear();
        image_node.color = NORMAL_BUTTON;
        button.set_changed();
    }
}
