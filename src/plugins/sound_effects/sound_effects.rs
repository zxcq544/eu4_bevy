use crate::{
    plugins::sound_effects::{
        components::sound_effects_player::SoundEffectsPlayer,
        resources::button_click_sound_effects::ButtonClickSoundEffects,
    },
    shared::settings::settings::Settings,
};
use bevy::{audio::Volume, prelude::*};

pub struct SoundEffectsPlugin;

#[derive(Default, Debug, Clone, Copy)]
pub enum ButtonSoundType {
    #[default]
    General,
    Back,
    Confirm,
}

#[derive(Event)]
pub struct PlayButtonClickSound(pub ButtonSoundType);

impl Plugin for SoundEffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(play_button_sound_observer);
    }
}

fn play_button_sound_observer(
    trigger: On<PlayButtonClickSound>,
    mut commands: Commands,
    sound_effects: Res<ButtonClickSoundEffects>,
    settings: Res<Settings>,
) {
    let sound_type = trigger.event().0;
    let required_sound = match sound_type {
        ButtonSoundType::General => sound_effects.button_click_general.clone(),
        ButtonSoundType::Back => sound_effects.button_click_back.clone(),
        ButtonSoundType::Confirm => sound_effects.button_click_confirm.clone(),
    };
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
}
