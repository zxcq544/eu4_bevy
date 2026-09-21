use bevy::{
    ecs::relationship::RelatedSpawnerCommands, prelude::*, render::render_resource::AsBindGroup,
    shader::ShaderRef,
};
use bevy_fluent::Localization;

use crate::{
    plugins::game_main::resources::game_ui_resources::GameUiResources,
    shared::fonts::fonts::FontHandles,
};

pub fn bottom_middle_block_entity_for_choose_country(
    bottom_middle_block: &mut RelatedSpawnerCommands<'_, ChildOf>,
    _localization_res: &Res<Localization>,
    _fonts: &Res<FontHandles>,
    game_ui_resources: &Res<GameUiResources>,
    ui_materials_res: Res<SharedUiMaterials>,
) {
    country_flags_block(bottom_middle_block, game_ui_resources, ui_materials_res);
    // country_shield_glow(bottom_middle_block, game_ui_resources);
    country_text_block(bottom_middle_block, game_ui_resources);
}

fn country_flags_block(
    bottom_middle_block: &mut RelatedSpawnerCommands<'_, ChildOf>,
    // localization_res: &Res<Localization>,
    // fonts: &Res<FontHandles>,
    game_ui_resources: &Res<GameUiResources>,
    ui_materials_res: Res<SharedUiMaterials>,
) {
    let num_flags = 11;
    // main block for flags
    bottom_middle_block
        .spawn((
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                width: Val::Percent(100.0),
                height: Val::Percent(30.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                // padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            // Outline {
            //     color: Color::srgb_from_array([0.9, 0.1, 0.1]),
            //     width: Val::Px(2.0),
            //     ..default()
            // },
        ))
        .with_children(|flags_block| {
            flags_block.spawn((
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(100.0 / num_flags as f32),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    // padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                Outline {
                    color: Color::srgb_from_array([0.9, 0.5, 0.1]),
                    width: Val::Px(1.0),
                    ..default()
                },
                ImageNode {
                    image: game_ui_resources.country_shield_frame.clone(),
                    image_mode: NodeImageMode::Stretch,
                    ..default()
                },
            ));
            flags_block.spawn((
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(100.0 / num_flags as f32),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    // padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                Outline {
                    color: Color::srgb_from_array([0.9, 0.5, 0.1]),
                    width: Val::Px(1.0),
                    ..default()
                },
                ImageNode {
                    image: game_ui_resources.test_country_flag.clone(),
                    image_mode: NodeImageMode::Stretch,
                    ..default()
                },
            ));
            flags_block.spawn((
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(100.0 / num_flags as f32),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    // padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                Outline {
                    color: Color::srgb_from_array([0.9, 0.5, 0.1]),
                    width: Val::Px(1.0),
                    ..default()
                },
                ImageNode {
                    image: game_ui_resources.country_shield_frame_mask.clone(),
                    image_mode: NodeImageMode::Stretch,
                    ..default()
                },
            ));
            flags_block.spawn((
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(100.0 / num_flags as f32),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    // padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                Outline {
                    color: Color::srgb_from_array([0.9, 0.5, 0.1]),
                    width: Val::Px(1.0),
                    ..default()
                },
                ImageNode {
                    image: game_ui_resources.country_shield_frame_green_glow.clone(),
                    image_mode: NodeImageMode::Stretch,
                    ..default()
                },
            ));
            // flag with material
            flags_block.spawn((
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(100.0 / num_flags as f32),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    // padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                MaterialNode(ui_materials_res.layered_ui.clone()),
                Outline {
                    color: Color::srgb_from_array([0.9, 0.9, 0.9]),
                    width: Val::Px(1.0),
                    ..default()
                },
            ));
        });
}

// fn country_shield_glow(
//     bottom_middle_block: &mut RelatedSpawnerCommands<'_, ChildOf>,
//     game_ui_resources: &Res<GameUiResources>,
// ) {
//     bottom_middle_block.spawn((
//         Node {
//             display: Display::Flex,
//             flex_direction: FlexDirection::Row,
//             width: Val::Percent(50.0),
//             height: Val::Percent(30.0),
//             justify_content: JustifyContent::Center,
//             align_items: AlignItems::Center,
//             // padding: UiRect::all(Val::Px(10.0)),
//             ..default()
//         },
//         Outline {
//             color: Color::srgb_from_array([0.9, 0.1, 0.1]),
//             width: Val::Px(2.0),
//             ..default()
//         },
//         ImageNode {
//             image: game_ui_resources.country_shield_frame_green_glow.clone(),
//             image_mode: NodeImageMode::Stretch,
//             ..default()
//         },
//         Text::new("flags shield block"),
//     ));
// }

fn country_text_block(
    bottom_middle_block: &mut RelatedSpawnerCommands<'_, ChildOf>,
    game_ui_resources: &Res<GameUiResources>,
) {
    bottom_middle_block.spawn((
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            width: Val::Percent(100.0),
            height: Val::Percent(70.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            // padding: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        Outline {
            color: Color::srgb_from_array([0.1, 0.9, 0.1]),
            width: Val::Px(2.0),
            ..default()
        },
        ImageNode {
            image: game_ui_resources
                .country_choice_bottom_middle_block_background_image
                .clone(),
            image_mode: NodeImageMode::Stretch,
            ..default()
        },
        Text::new("text block"),
    ));
}

#[derive(Resource)]
pub struct SharedUiMaterials {
    pub layered_ui: Handle<CustomUiMaterial>,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomUiMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub base_texture: Handle<Image>,

    #[texture(2)]
    #[sampler(3)]
    pub overlay_texture: Handle<Image>,

    #[texture(4)]
    #[sampler(5)]
    pub mask_texture: Handle<Image>,
}

impl UiMaterial for CustomUiMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/layered_ui.wgsl".into()
    }
}
