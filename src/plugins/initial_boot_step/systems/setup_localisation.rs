use crate::plugins::initial_boot_step::resources::locale_folder::LocaleFolder;
use bevy::prelude::*;
use bevy_fluent::LocalizationBuilder;

pub fn setup_localisation(
    commands: &mut Commands,
    localization_builder: LocalizationBuilder,
    locale_folder_res: Res<LocaleFolder>,
    asset_server: Res<AssetServer>,
) {
    if asset_server.is_loaded_with_dependencies(&locale_folder_res.folder) {
        let localization = localization_builder.build(&locale_folder_res.folder);
        commands.remove_resource::<LocaleFolder>();
        commands.insert_resource(localization);
    }
}
