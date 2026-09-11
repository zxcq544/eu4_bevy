use audio_channel::AudioChannel;
use bevy::{prelude::*, ui_widgets::{ValueChange, observe, slider_self_update}};
use settings::Settings;
use crate::plugins::options::components::options_middle_block::audio_slider_widget::options_slider_base::slider;

pub fn sound_volume_slider(
    settings: &ResMut<Settings>,
    audio_channel: AudioChannel,
) -> impl Bundle {
    (
        audio_channel,
        slider(0.0, 1.0, audio_channel.get(&settings)),
        observe(slider_self_update),
        observe(
            move |change: On<ValueChange<f32>>, mut settings: ResMut<Settings>| {
                audio_channel.set(&mut settings, change.value);
            },
        ),
    )
}
