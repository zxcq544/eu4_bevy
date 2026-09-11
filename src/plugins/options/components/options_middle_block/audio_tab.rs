use crate::plugins::{
    game_main::resources::game_ui_resources::GameUiResources,
    options::components::options_middle_block::audio_slider_widget::sound_volume_slider::sound_volume_slider,
};
use audio_channel::AudioChannel;
use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};
use bevy_fluent::Localization;
use fluent_content::Content;
use fonts::FontHandles;
use settings::Settings;

#[derive(Component, Clone, Default)]
pub struct OptionsUiAudioTab;

pub fn audio_tab(
    options_middle_block: &mut RelatedSpawnerCommands<'_, ChildOf>,
    settings: ResMut<Settings>,
    localization_res: &Res<Localization>,
    fonts: &Res<FontHandles>,
    ui_resources: &Res<GameUiResources>,
) {
    options_middle_block
        .spawn((
            OptionsUiAudioTab,
            Node {
                display: Display::None,
                // flex_direction: FlexDirection::Row,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                // justify_content: JustifyContent::FlexStart,
                // align_items: AlignItems::Center,
                // bottom: Val::Px(3.0),
                grid_template_columns: RepeatedGridTrack::flex(2, 1.0),
                grid_template_rows: RepeatedGridTrack::flex(3, 1.0),
                row_gap: Val::Percent(1.0),
                ..default()
            },
            // Outline {
            //     color: Color::srgb_from_array([0.9, 0.9, 0.1]),
            //     width: Val::Px(2.0),
            //     ..default()
            // },
        ))
        .with_children(|grid_builder| {
            grid_builder
                .spawn((
                    Node {
                        display: Display::Grid,
                        // flex_direction: FlexDirection::Row,
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        // bottom: Val::Px(3.0),
                        ..default()
                    },
                    Outline {
                        color: Color::srgb_from_array([0.9, 0.9, 0.1]),
                        width: Val::Px(2.0),
                        ..default()
                    },
                ))
                .with_children(|left_block_top| {
                    top_left_block(left_block_top, &localization_res, &fonts);
                });
            grid_builder
                .spawn((
                    Node {
                        display: Display::Grid,
                        // flex_direction: FlexDirection::Column,
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::left(Val::Percent(12.5)).with_right(Val::Percent(12.5)),
                        // bottom: Val::Px(3.0),
                        ..default()
                    },
                    Outline {
                        color: Color::srgb_from_array([0.1, 0.1, 0.9]),
                        width: Val::Px(2.0),
                        ..default()
                    },
                ))
                .with_children(|right_block_top| {
                    right_block_top.spawn(sound_volume_slider(
                        &settings,
                        AudioChannel::Master,
                        &ui_resources,
                    ));
                });
            grid_builder
                .spawn((
                    Node {
                        display: Display::Grid,
                        // flex_direction: FlexDirection::Column,
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        // bottom: Val::Px(3.0),
                        ..default()
                    },
                    Outline {
                        color: Color::srgb_from_array([0.1, 0.1, 0.9]),
                        width: Val::Px(2.0),
                        ..default()
                    },
                ))
                .with_children(|middle_left| {
                    middle_left_block(middle_left, &localization_res, &fonts);
                });
            grid_builder
                .spawn((
                    Node {
                        display: Display::Grid,
                        // flex_direction: FlexDirection::Column,
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::left(Val::Percent(12.5)).with_right(Val::Percent(12.5)),
                        // bottom: Val::Px(3.0),
                        ..default()
                    },
                    Outline {
                        color: Color::srgb_from_array([0.1, 0.1, 0.9]),
                        width: Val::Px(2.0),
                        ..default()
                    },
                ))
                .with_children(|middle_right_block| {
                    middle_right_block.spawn(sound_volume_slider(
                        &settings,
                        AudioChannel::Music,
                        &ui_resources,
                    ));
                });
            grid_builder
                .spawn((
                    Node {
                        display: Display::Grid,
                        // flex_direction: FlexDirection::Column,
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        // bottom: Val::Px(3.0),
                        ..default()
                    },
                    Outline {
                        color: Color::srgb_from_array([0.1, 0.1, 0.9]),
                        width: Val::Px(2.0),
                        ..default()
                    },
                ))
                .with_children(|bottom_left| {
                    bottom_left_block(bottom_left, &localization_res, &fonts);
                });
            grid_builder
                .spawn((
                    Node {
                        display: Display::Grid,
                        // flex_direction: FlexDirection::Column,
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::left(Val::Percent(12.5)).with_right(Val::Percent(12.5)),
                        // bottom: Val::Px(3.0),
                        ..default()
                    },
                    Outline {
                        color: Color::srgb_from_array([0.1, 0.1, 0.9]),
                        width: Val::Px(2.0),
                        ..default()
                    },
                ))
                .with_children(|bottom_right_block| {
                    bottom_right_block.spawn(sound_volume_slider(
                        &settings,
                        AudioChannel::Sfx,
                        &ui_resources,
                    ));
                });
        });
}

fn top_left_block(
    left_block_top: &mut RelatedSpawnerCommands<'_, ChildOf>,
    localisation_res: &Res<Localization>,
    fonts: &Res<FontHandles>,
) {
    let label = localisation_res.content("master_volume").expect(&format!(
        "missing master_volume in localisation files {:?}",
        localisation_res
    ));
    left_block_top.spawn((
        Text::new(label),
        TextFont {
            font_size: FontSize::Px(20.0),
            font: FontSource::Handle(fonts.button_font.clone()),
            ..default()
        },
        TextLayout {
            justify: Justify::Center,
            ..default()
        },
    ));
}

fn middle_left_block(
    middle_left_block: &mut RelatedSpawnerCommands<'_, ChildOf>,
    localisation_res: &Res<Localization>,
    fonts: &Res<FontHandles>,
) {
    let label = localisation_res.content("music_volume").expect(&format!(
        "missing music_volume in localisation files {:?}",
        localisation_res
    ));
    middle_left_block.spawn((
        Text::new(label),
        TextFont {
            font_size: FontSize::Px(20.0),
            font: FontSource::Handle(fonts.button_font.clone()),
            ..default()
        },
        TextLayout {
            justify: Justify::Center,
            ..default()
        },
    ));
}

fn bottom_left_block(
    bottom_left_block: &mut RelatedSpawnerCommands<'_, ChildOf>,
    localisation_res: &Res<Localization>,
    fonts: &Res<FontHandles>,
) {
    let label = localisation_res.content("sfx_volume").expect(&format!(
        "missing sfx_volume in localisation files {:?}",
        localisation_res
    ));
    bottom_left_block.spawn((
        Text::new(label),
        TextFont {
            font_size: FontSize::Px(20.0),
            font: FontSource::Handle(fonts.button_font.clone()),
            ..default()
        },
        TextLayout {
            justify: Justify::Center,
            ..default()
        },
    ));
}
