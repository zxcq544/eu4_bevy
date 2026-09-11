use crate::plugins::{
    game_main::resources::game_ui_resources::GameUiResources,
    options::components::options_middle_block::audio_slider_widget::options_slider_base::slider,
};
use audio_channel::AudioChannel;
use bevy::{
    prelude::*,
    ui_widgets::{ValueChange, observe, slider_self_update},
};
use settings::Settings;

pub fn sound_volume_slider(
    settings: &ResMut<Settings>,
    audio_channel: AudioChannel,
    ui_resources: &Res<GameUiResources>,
) -> impl Bundle {
    (
        audio_channel,
        slider(0.0, 1.0, audio_channel.get(&settings), ui_resources),
        observe(slider_self_update),
        observe(
            move |change: On<ValueChange<f32>>, mut settings: ResMut<Settings>| {
                audio_channel.set(&mut settings, change.value);
            },
        ),
    )
}
