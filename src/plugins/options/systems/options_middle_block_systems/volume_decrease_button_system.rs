use audio_channel::AudioChannel;
use bevy::audio::Volume;
use bevy::prelude::*;
use bevy::{
    ecs::{
        entity::Entity,
        system::{Commands, Query, Res, ResMut},
    },
    input_focus::{FocusCause, InputFocus},
    ui::Interaction,
};
use settings::Settings;

use crate::plugins::options::components::options_middle_block::audio_slider_widget::slider_left_arrow_button::OptionsUIAudioSliderButtonLeft;
use crate::plugins::sound_effects::components::sound_effects_player::SoundEffectsPlayer;
use crate::plugins::sound_effects::resources::button_click_sound_effects::ButtonClickSoundEffects;

pub const NORMAL_BUTTON: Color = Color::srgb(1.0, 1.0, 1.0);
pub const HOVERED_BUTTON: Color = Color::srgb(1.15, 1.15, 1.15);
pub const PRESSED_BUTTON: Color = Color::srgb(0.85, 0.85, 0.85);

#[derive(Message)]
pub struct VolumeDecreasedMessage {
    pub audio_channel: AudioChannel,
}

pub fn volume_decrease_button_system(
    mut message_writer: MessageWriter<VolumeDecreasedMessage>,
    mut commands: Commands,
    settings: Res<Settings>,
    sound_effects: Res<ButtonClickSoundEffects>,
    mut input_focus: ResMut<InputFocus>,
    mut interaction_query: Query<
        (
            Entity,
            &mut OptionsUIAudioSliderButtonLeft,
            &Interaction,
            &mut ImageNode,
            &AudioChannel,
        ),
        Changed<Interaction>,
    >,
) {
    for (entity, mut button, interaction, mut image_node, audio_channel) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                input_focus.set(entity, FocusCause::Pressed);
                // node.border = UiRect::all(Val::Px(20.0));
                image_node.color = PRESSED_BUTTON;
                // The accessibility system's only update the button's state when the `Button` component is marked as changed.
                button.set_changed();

                // Sound effect logic
                let required_sound = sound_effects.button_click_general.clone();
                commands.spawn((
                    SoundEffectsPlayer,
                    AudioPlayer::new(required_sound),
                    PlaybackSettings {
                        mode: bevy::audio::PlaybackMode::Despawn,
                        volume: Volume::Linear(settings.volume_settings.get_sfx_volume()),
                        speed: 1.0,
                        paused: false,
                        muted: false,
                        ..default()
                    },
                ));
                message_writer.write(VolumeDecreasedMessage {
                    audio_channel: audio_channel.clone(),
                });
                info!("Volume decreased click {:?}", audio_channel);
            }
            Interaction::Hovered => {
                input_focus.set(entity, FocusCause::Pressed);
                // node.border = UiRect::all(Val::Px(10.0));
                image_node.color = HOVERED_BUTTON;
                button.set_changed();
            }
            Interaction::None => {
                input_focus.clear();
                // node.border = UiRect::all(Val::Px(0.0));
                image_node.color = NORMAL_BUTTON;
                button.set_changed();
            }
        }
    }
}
