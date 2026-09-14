use crate::plugins::options::{
    components::options_middle_block::audio_slider_widget::options_slider_base::OptionUiAudioSlider,
    systems::options_middle_block_systems::volume_decrease_button_system::VolumeDecreasedMessage,
};
use audio_channel::AudioChannel;
use bevy::{
    prelude::*,
    ui_widgets::{SetSliderValue, SliderValue, SliderValueChange},
};

pub fn volume_decreased_message_reader(
    mut message_reader: MessageReader<VolumeDecreasedMessage>,
    mut commands: Commands,
    slider_query: Query<(Entity, &AudioChannel, &SliderValue), With<OptionUiAudioSlider>>,
) {
    for message in message_reader.read() {
        let target_slider = slider_query
            .iter()
            .find(|(_, channel, _)| **channel == message.audio_channel);

        if let Some((entity, _, slider_value)) = target_slider {
            decrease_value(&mut commands, entity, slider_value);
            info!("Volume decreased for channel: {:?}", message.audio_channel);
        }
    }
}

fn decrease_value(commands: &mut Commands, slider_entity: Entity, slider_value: &SliderValue) {
    let new_value = (slider_value.0 - 0.1).clamp(0.0, 1.0);

    commands.trigger(SetSliderValue {
        entity: slider_entity,
        change: SliderValueChange::Absolute(new_value),
    });
}
