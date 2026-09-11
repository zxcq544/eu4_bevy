use crate::plugins::{
    music_player::music_player::BackgroundMusicPlayer,
    sound_effects::components::sound_effects_player::SoundEffectsPlayer,
};
use bevy::{audio::Volume, prelude::*};
use settings::Settings;

pub fn audio_volume_update_system(
    settings: Res<Settings>,
    mut sound_effects_player_query: Query<
        &mut AudioSink,
        (With<SoundEffectsPlayer>, Without<BackgroundMusicPlayer>),
    >,
    mut background_music_player_query: Query<
        &mut AudioSink,
        (With<BackgroundMusicPlayer>, Without<SoundEffectsPlayer>),
    >,
) {
    if !settings.is_changed() {
        return;
    }
    for mut sound_effects_audio_sink in &mut sound_effects_player_query {
        sound_effects_audio_sink
            .set_volume(Volume::Linear(settings.volume_settings.get_sfx_volume()));
    }
    for mut background_music_audio_sink in &mut background_music_player_query {
        background_music_audio_sink
            .set_volume(Volume::Linear(settings.volume_settings.get_music_volume()));
    }
}
