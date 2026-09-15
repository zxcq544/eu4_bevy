use crate::{
    plugins::choose_country::components::choose_country_ui_main_entity::ChooseCountryUiMainEntity,
    shared::fonts::fonts::FontHandles,
};
use bevy::prelude::*;
use bevy_fluent::Localization;

pub fn spawn_choose_country_ui_main_entity(
    commands: Commands,
    localization_res: Res<Localization>,
    fonts: Res<FontHandles>,
) {
    info!("Spawning choose country ui main entity");
    ChooseCountryUiMainEntity::spawn_using_commands(commands, &localization_res, &fonts);
}
