use bevy::audio::Volume;
use bevy::prelude::*;
use bevy::ui_widgets::{SetSliderValue, SliderValueChange};
use bevy::{
    ecs::{
        entity::Entity,
        system::{Commands, Query, Res, ResMut},
    },
    input_focus::{FocusCause, InputFocus},
    ui::Interaction,
};
use crate::plugins::options::components::options_middle_block::audio_slider_widget::options_slider_base::OptionUiAudioSlider;
use crate::plugins::options::components::options_middle_block::audio_slider_widget::slider_right_arrow_button::OptionsUIAudioSliderButtonRight;
use crate::plugins::sound_effects::components::sound_effects_player::SoundEffectsPlayer;
use crate::plugins::sound_effects::resources::button_click_sound_effects::ButtonClickSoundEffects;
use crate::shared::audio_channel::audio_channel::AudioChannel;
use crate::shared::settings::settings::Settings;

pub const NORMAL_BUTTON: Color = Color::srgb(1.0, 1.0, 1.0);
pub const HOVERED_BUTTON: Color = Color::srgb(1.15, 1.15, 1.15);
pub const PRESSED_BUTTON: Color = Color::srgb(0.85, 0.85, 0.85);

pub fn volume_increase_button_system(
    mut commands: Commands,
    settings: Res<Settings>,
    sound_effects: Res<ButtonClickSoundEffects>,
    mut input_focus: ResMut<InputFocus>,
    mut interaction_query: Query<
        (
            Entity,
            &mut OptionsUIAudioSliderButtonRight,
            &Interaction,
            &mut ImageNode,
            &AudioChannel,
        ),
        Changed<Interaction>,
    >,
    sliders_query: Query<(Entity, &AudioChannel), With<OptionUiAudioSlider>>,
) {
    for (entity, mut button, interaction, mut image_node, audio_channel_of_button) in
        &mut interaction_query
    {
        match *interaction {
            Interaction::Pressed => {
                input_focus.set(entity, FocusCause::Pressed);
                image_node.color = PRESSED_BUTTON;
                // The accessibility system's only update the button's state when the `Button` component is marked as changed.
                button.set_changed();

                // Sound effect logic
                commands.spawn((
                    SoundEffectsPlayer,
                    AudioPlayer::new(sound_effects.button_click_general.clone()),
                    PlaybackSettings {
                        mode: bevy::audio::PlaybackMode::Despawn,
                        volume: Volume::Linear(settings.volume_settings.get_sfx_volume()),
                        ..default()
                    },
                ));
                // find slider with same audio channel as button's audio channel
                for (slider_entity, audio_channel_of_slider) in &sliders_query {
                    if *audio_channel_of_slider == *audio_channel_of_button {
                        let current_value = audio_channel_of_slider.get(&settings);
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
