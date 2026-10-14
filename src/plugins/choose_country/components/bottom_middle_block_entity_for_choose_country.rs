use bevy::{
    ecs::relationship::RelatedSpawnerCommands,
    material::descriptor::RenderPipelineDescriptor,
    prelude::*,
    render::render_resource::{AsBindGroup, BlendState},
    shader::ShaderRef,
};
use bevy_fluent::Localization;

use crate::{
    plugins::game_main::resources::game_ui_resources::GameUiResources,
    shared::{country::country::Country, fonts::fonts::FontHandles},
};

pub fn bottom_middle_block_entity_for_choose_country(
    bottom_middle_block: &mut RelatedSpawnerCommands<'_, ChildOf>,
    _localization_res: &Res<Localization>,
    _fonts: &Res<FontHandles>,
    game_ui_resources: &Res<GameUiResources>,
    ui_materials_res: ResMut<Assets<CustomUiMaterial>>,
    ui_materials_with_glow_res: ResMut<Assets<CustomUiMaterialWithGlow>>,
) {
    country_flags_block(
        bottom_middle_block,
        game_ui_resources,
        ui_materials_res,
        ui_materials_with_glow_res,
    );
    // country_shield_glow(bottom_middle_block, game_ui_resources);
    country_text_block(bottom_middle_block, game_ui_resources);
}

fn country_flags_block(
    bottom_middle_block: &mut RelatedSpawnerCommands<'_, ChildOf>,
    // localization_res: &Res<Localization>,
    // fonts: &Res<FontHandles>,
    game_ui_resources: &Res<GameUiResources>,
    mut ui_materials: ResMut<Assets<CustomUiMaterial>>,
    mut ui_materials_with_glow: ResMut<Assets<CustomUiMaterialWithGlow>>,
) {
    let countries_recommended_for_main_menu = CountriesRecommendedForMainMenu {
        countries: vec![
            Country {
                name: "Austria".to_string(),
                flag_index: 0,
                short_name_tag_3_letters: "AUT".to_string(),
                recommended_in_menu: Some(true),
                index_in_menu: Some(0),
            },
            Country {
                name: "Sweden".to_string(),
                flag_index: 1,
                short_name_tag_3_letters: "SWE".to_string(),
                recommended_in_menu: Some(true),
                index_in_menu: Some(1),
            },
            Country {
                name: "Bulgaria".to_string(),
                flag_index: 2,
                short_name_tag_3_letters: "BUL".to_string(),
                recommended_in_menu: Some(true),
                index_in_menu: Some(2),
            },
        ],
    };
    let countries_for_main_menu = CountriesForMainMenu {
        countries: vec![
            Country {
                name: "Croatia".to_string(),
                flag_index: 3,
                short_name_tag_3_letters: "CRO".to_string(),
                recommended_in_menu: Some(true),
                index_in_menu: Some(3),
            },
            Country {
                name: "Finland".to_string(),
                flag_index: 4,
                short_name_tag_3_letters: "FIN".to_string(),
                recommended_in_menu: Some(true),
                index_in_menu: Some(4),
            },
            Country {
                name: "Denmark".to_string(),
                flag_index: 5,
                short_name_tag_3_letters: "DAN".to_string(),
                recommended_in_menu: Some(true),
                index_in_menu: Some(5),
            },
            Country {
                name: "Serbia".to_string(),
                flag_index: 6,
                short_name_tag_3_letters: "SER".to_string(),
                recommended_in_menu: Some(false),
                index_in_menu: Some(6),
            },
            Country {
                name: "France".to_string(),
                flag_index: 7,
                short_name_tag_3_letters: "FRA".to_string(),
                recommended_in_menu: Some(false),
                index_in_menu: Some(7),
            },
            Country {
                name: "Poland".to_string(),
                flag_index: 8,
                short_name_tag_3_letters: "POL".to_string(),
                recommended_in_menu: Some(false),
                index_in_menu: Some(8),
            },
            Country {
                name: "Aragon".to_string(),
                flag_index: 9,
                short_name_tag_3_letters: "ARA".to_string(),
                recommended_in_menu: Some(false),
                index_in_menu: Some(9),
            },
            Country {
                name: "Genoa".to_string(),
                flag_index: 10,
                short_name_tag_3_letters: "GEN".to_string(),
                recommended_in_menu: Some(false),
                index_in_menu: Some(10),
            },
        ],
    };
    let num_flags = 11;
    let flag_index_1 = 0;
    let flag_index_2 = 1026;
    // main block for flags
    bottom_middle_block
        .spawn((
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                width: Val::Percent(100.0),
                height: Val::Percent(32.0),
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
            for country in &countries_recommended_for_main_menu.countries {
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
                    MaterialNode(ui_materials_with_glow.add(CustomUiMaterialWithGlow {
                        flag_texture: game_ui_resources.country_flags_atlas.clone(),
                        shield_texture: game_ui_resources.country_shield_frame.clone(),
                        mask_texture: game_ui_resources.country_shield_frame_mask.clone(),
                        glow_texture: game_ui_resources.country_shield_frame_green_glow.clone(),
                        hover_color: LinearRgba::new(1.0, 1.0, 1.0, 1.0),
                        flag_index: country.flag_index,
                    })),
                    // Outline {
                    //     color: Color::srgb_from_array([0.9, 0.9, 0.9]),
                    //     width: Val::Px(1.0),
                    //     ..default()
                    // },
                ));
            }
            for country in &countries_for_main_menu.countries {
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
                    MaterialNode(ui_materials.add(CustomUiMaterial {
                        flag_texture: game_ui_resources.country_flags_atlas.clone(),
                        shield_texture: game_ui_resources.country_shield_frame.clone(),
                        mask_texture: game_ui_resources.country_shield_frame_mask.clone(),
                        hover_color: LinearRgba::new(1.0, 1.0, 1.0, 1.0),
                        flag_index: country.flag_index,
                    })),
                    // Outline {
                    //     color: Color::srgb_from_array([0.9, 0.9, 0.9]),
                    //     width: Val::Px(1.0),
                    //     ..default()
                    // },
                ));
            }
        });
}

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

// #[derive(Resource)]
// pub struct SharedUiMaterials {
//     pub layered_ui: Handle<CustomUiMaterial>,
// }

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomUiMaterial {
    #[texture(0, dimension = "2d_array")]
    #[sampler(1)]
    pub flag_texture: Handle<Image>,

    #[texture(2)]
    #[sampler(3)]
    pub shield_texture: Handle<Image>,

    #[texture(4)]
    #[sampler(5)]
    pub mask_texture: Handle<Image>,

    #[uniform(6)]
    pub hover_color: LinearRgba,

    #[uniform(7)]
    pub flag_index: u32,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomUiMaterialWithGlow {
    #[texture(0, dimension = "2d_array")]
    #[sampler(1)]
    pub flag_texture: Handle<Image>,

    #[texture(2)]
    #[sampler(3)]
    pub shield_texture: Handle<Image>,

    #[texture(4)]
    #[sampler(5)]
    pub mask_texture: Handle<Image>,

    #[texture(6)]
    #[sampler(7)]
    pub glow_texture: Handle<Image>,

    #[uniform(8)]
    pub hover_color: LinearRgba,

    #[uniform(9)]
    pub flag_index: u32,
}

impl UiMaterial for CustomUiMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/shield_flag_premul_alpha.wgsl".into()
    }
    fn specialize(descriptor: &mut RenderPipelineDescriptor, _key: UiMaterialKey<Self>) {
        if let Some(fragment) = descriptor.fragment.as_mut() {
            for target in fragment.targets.iter_mut() {
                if let Some(target) = target {
                    target.blend = Some(BlendState::PREMULTIPLIED_ALPHA_BLENDING);
                }
            }
        }
    }
}

impl UiMaterial for CustomUiMaterialWithGlow {
    fn fragment_shader() -> ShaderRef {
        "shaders/shield_flag_with_glow_premul_alpha.wgsl".into()
    }
    fn specialize(descriptor: &mut RenderPipelineDescriptor, _key: UiMaterialKey<Self>) {
        if let Some(fragment) = descriptor.fragment.as_mut() {
            for target in fragment.targets.iter_mut() {
                if let Some(target) = target {
                    target.blend = Some(BlendState::PREMULTIPLIED_ALPHA_BLENDING);
                }
            }
        }
    }
}

// fn row_and_column_to_flag_index(row: u32, column: u32) -> u32 {
//     row * 32 + column
// }

struct CountriesRecommendedForMainMenu {
    pub countries: Vec<Country>,
}

struct CountriesForMainMenu {
    pub countries: Vec<Country>,
}

// struct Country {
//     pub name: String,
//     pub short_name_tag_3_letters: String,
//     flag_index: u32,
// }
