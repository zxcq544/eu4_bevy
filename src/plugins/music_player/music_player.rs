use bevy::{audio::Volume, prelude::*};

// #[derive(Resource)]
// pub struct MusicPlayer {
//     pub tracks: Vec<Handle<AudioSource>>,
//     pub current_index: usize,
//     pub volume: f32,
//     pub loop_tracks: bool,
//     pub current_entity: Option<Entity>,
//     pub track_paths: Vec<String>,
//     is_paused: bool,
// }

pub struct MusicPlayerPlugin;

impl Plugin for MusicPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // let track_paths = vec![
    //     "music/maintheme.ogg".to_string(),
    //     "music/mood_discovery.ogg".to_string(),
    //     "music/openseas.ogg".to_string(),
    // ];

    // let tracks = track_paths
    //     .iter()
    //     .map(|path| asset_server.load(path))
    //     .collect::<Vec<_>>();

    // commands.insert_resource(MusicPlayer {
    //     tracks,
    //     current_index: 0,
    //     volume: 0.5,
    //     loop_tracks: true,
    //     current_entity: None,
    //     track_paths,
    //     is_paused: false,
    // });
    let track = asset_server.load::<AudioSource>("music/maintheme.ogg");
    commands.spawn((
        AudioPlayer(track),
        PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Loop,
            volume: Volume::Linear(0.5),
            speed: 1.0,
            paused: false,
            muted: false,
            ..default()
        },
    ));
}
