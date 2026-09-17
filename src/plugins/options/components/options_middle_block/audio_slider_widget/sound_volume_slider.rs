use crate::{
    plugins::{
        game_main::resources::game_ui_resources::GameUiResources,
        options::components::options_middle_block::audio_slider_widget::options_slider_base::slider,
    },
    shared::{audio_channel::audio_channel::AudioChannelUIMarker, settings::settings::Settings},
};
use bevy::{
    prelude::*,
    ui_widgets::{ValueChange, observe, slider_self_update},
};

pub fn sound_volume_slider(
    settings: &ResMut<Settings>,
    audio_channel: AudioChannelUIMarker,
    ui_resources: &Res<GameUiResources>,
) -> impl Bundle {
    (
        audio_channel,
        slider(
            0.0,
            1.0,
            audio_channel.get_volume_from_settings(&settings),
            ui_resources,
        ),
        observe(slider_self_update),
        observe(
            move |change: On<ValueChange<f32>>, mut settings: ResMut<Settings>| {
                audio_channel.set_volume_in_settings(&mut settings, change.value);
            },
        ),
    )
}
