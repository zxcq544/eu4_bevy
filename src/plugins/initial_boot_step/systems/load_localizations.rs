use crate::plugins::initial_boot_step::resources::locale_folder::LocaleFolder;
use bevy::prelude::*;

// TODO: care to not load all translations and load only the one that is set in settings
pub fn load_localizations(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Loading localizations");
    let localisation_folder_handle = asset_server.load_folder("locales");
    commands.insert_resource(LocaleFolder {
        folder: localisation_folder_handle,
    });
}
