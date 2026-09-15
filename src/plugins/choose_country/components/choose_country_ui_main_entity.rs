use bevy::prelude::*;
use bevy_fluent::Localization;

use crate::shared::fonts::fonts::FontHandles;

#[derive(Component, Clone, Default)]
pub struct ChooseCountryUiMainEntity;

impl ChooseCountryUiMainEntity {
    pub fn spawn_using_commands(
        mut commands: Commands,
        _localization_res: &Res<Localization>,
        _fonts: &Res<FontHandles>,
    ) {
        commands.spawn((
            ChooseCountryUiMainEntity,
            Node {
                display: Display::None,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            Visibility::Hidden, // Spawn as hidden because we don't despawn ingame ui elements
            BackgroundColor(Color::NONE),
        ));
        commands.spawn(Text::new("chooose country"));
    }
}
