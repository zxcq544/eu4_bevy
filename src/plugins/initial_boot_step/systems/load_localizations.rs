use crate::plugins::initial_boot_step::resources::locale_folder::LocaleFolder;
use bevy::prelude::*;

// TODO: care to not load all translations and load only the one that is set in settings
pub fn load_localizations(mut commands: Commands, asset_server: Res<AssetServer>) {
    let localisation_folder_handle = asset_server.load_folder("locales");
    commands.insert_resource(LocaleFolder {
        folder: localisation_folder_handle,
    });
}

// pub fn update(
//     mut commands: Commands,
//     localization_builder: LocalizationBuilder,
//     asset_server: Res<AssetServer>,
//     mut next_state: ResMut<NextState<GameState>>,
//     locale_folder: Res<LocaleFolder>,
// ) {
//     if let Some(LoadState::Loaded) = asset_server.get_load_state(&locale_folder.0) {
//         let localization = localization_builder.build(&locale_folder.0);
//         commands.remove_resource::<LocaleFolder>();
//         commands.insert_resource(localization);
//         next_state.set(GameState::Menu);
//     }
// }
