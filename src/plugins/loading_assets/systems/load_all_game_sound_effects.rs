use crate::plugins::sound_effects::resources::button_click_sound_effects::ButtonClickSoundEffects;
use bevy::prelude::*;

pub fn load_all_game_sound_effects(asset_server: Res<AssetServer>, mut commands: Commands) {
    let button_click_sound_effects = ButtonClickSoundEffects {
        button_click_general: asset_server.load("sound/general_button_click.wav"),
        button_click_ok: asset_server.load("sound/general_ok_button_click.wav"),
        button_click_back: asset_server.load("sound/general_back_button_click.wav"),
    };
    commands.insert_resource(button_click_sound_effects);
    info!("Loading sound effects");
}
