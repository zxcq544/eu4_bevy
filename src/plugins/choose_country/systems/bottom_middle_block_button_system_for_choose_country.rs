use crate::{
    plugins::{
        choose_country::components::bottom_middle_block_entity_for_choose_country::{
            BottomMiddleBlockButtonForChooseCountry, CustomUiMaterial, CustomUiMaterialWithGlow,
        },
        sound_effects::{
            components::sound_effects_player::SoundEffectsPlayer,
            resources::button_click_sound_effects::ButtonClickSoundEffects,
        },
    },
    shared::settings::settings::Settings,
};
use bevy::{
    audio::Volume,
    input_focus::{FocusCause, InputFocus},
    prelude::*,
};

pub const NORMAL_BUTTON: LinearRgba = LinearRgba::new(1.0, 1.0, 1.0, 1.0);
pub const HOVERED_BUTTON: LinearRgba = LinearRgba::new(1.5, 1.5, 1.5, 1.0);
pub const PRESSED_BUTTON: LinearRgba = LinearRgba::new(0.5, 0.5, 0.5, 1.0);

// TODO: uncomment for bevy 0.20
pub fn on_country_choice_button_hover(
    // TODO: uncomment for bevy 0.20
    // trigger: Trigger<PointerOver>,
    trigger: On<Pointer<Over>>,
    mut query: Query<
        (
            Option<&MaterialNode<CustomUiMaterial>>,
            Option<&MaterialNode<CustomUiMaterialWithGlow>>,
        ),
        With<BottomMiddleBlockButtonForChooseCountry>,
    >,
    mut mat_res: ResMut<Assets<CustomUiMaterial>>,
    mut mat_glow_res: ResMut<Assets<CustomUiMaterialWithGlow>>,
) {
    if let Ok((mat, mat_glow)) = query.get_mut(trigger.event_target()) {
        update_material_color(
            mat,
            mat_glow,
            HOVERED_BUTTON,
            &mut mat_res,
            &mut mat_glow_res,
        );
    }
}

pub fn on_country_choice_button_unhover(
    // TODO: uncomment for bevy 0.20
    // trigger: Trigger<PointerOut>,
    trigger: On<Pointer<Out>>,
    mut input_focus: ResMut<InputFocus>,
    mut query: Query<
        (
            Option<&MaterialNode<CustomUiMaterial>>,
            Option<&MaterialNode<CustomUiMaterialWithGlow>>,
        ),
        With<BottomMiddleBlockButtonForChooseCountry>,
    >,
    mut mat_res: ResMut<Assets<CustomUiMaterial>>,
    mut mat_glow_res: ResMut<Assets<CustomUiMaterialWithGlow>>,
) {
    if let Ok((mat, mat_glow)) = query.get_mut(trigger.event_target()) {
        input_focus.clear();
        update_material_color(
            mat,
            mat_glow,
            NORMAL_BUTTON,
            &mut mat_res,
            &mut mat_glow_res,
        );
    }
}

pub fn on_country_choice_button_down(
    // TODO: uncomment for bevy 0.20
    // trigger: Trigger<PointerDown>,
    trigger: On<Pointer<Press>>,
    mut input_focus: ResMut<InputFocus>,
    mut query: Query<(
        &mut BottomMiddleBlockButtonForChooseCountry,
        Option<&MaterialNode<CustomUiMaterial>>,
        Option<&MaterialNode<CustomUiMaterialWithGlow>>,
    )>,
    mut mat_res: ResMut<Assets<CustomUiMaterial>>,
    mut mat_glow_res: ResMut<Assets<CustomUiMaterialWithGlow>>,
) {
    let entity = trigger.event_target();
    if let Ok((mut button, mat, mat_glow)) = query.get_mut(entity) {
        input_focus.set(entity, FocusCause::Pressed);
        button.set_changed();

        update_material_color(
            mat,
            mat_glow,
            PRESSED_BUTTON,
            &mut mat_res,
            &mut mat_glow_res,
        );
    }
}

pub fn on_country_choice_button_click_release(
    // TODO: uncomment for bevy 0.20
    // trigger: Trigger<PointerClick>,
    trigger: On<Pointer<Click>>,
    mut commands: Commands,
    sound_effects: Res<ButtonClickSoundEffects>,
    settings: Res<Settings>,
    mut query: Query<
        (
            Option<&MaterialNode<CustomUiMaterial>>,
            Option<&MaterialNode<CustomUiMaterialWithGlow>>,
        ),
        With<BottomMiddleBlockButtonForChooseCountry>,
    >,
    mut mat_res: ResMut<Assets<CustomUiMaterial>>,
    mut mat_glow_res: ResMut<Assets<CustomUiMaterialWithGlow>>,
) {
    let entity = trigger.event_target();

    // Return button color Hover, because the cursor is still over the button at the moment of Pointer Up
    if let Ok((mat, mat_glow)) = query.get_mut(entity) {
        update_material_color(
            mat,
            mat_glow,
            HOVERED_BUTTON,
            &mut mat_res,
            &mut mat_glow_res,
        );
    }

    // --- BUTTON ACTION LOGIC (Works only on Pointer Up) ---
    let required_sound = sound_effects.button_click_general.clone();
    commands.spawn((
        SoundEffectsPlayer,
        AudioPlayer::new(required_sound),
        PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Despawn,
            volume: Volume::Linear(
                settings
                    .volume_settings
                    .get_sfx_volume_multiplied_by_master(),
            ),
            ..default()
        },
    ));

    info!("Click released! Action performed when releasing the mouse button.");
    // next_state.set(GameState::MainMenu);
}

fn update_material_color(
    mat_node: Option<&MaterialNode<CustomUiMaterial>>,
    mat_glow_node: Option<&MaterialNode<CustomUiMaterialWithGlow>>,
    color: LinearRgba,
    material_without_glow_res: &mut Assets<CustomUiMaterial>,
    material_with_glow_res: &mut Assets<CustomUiMaterialWithGlow>,
) {
    if let Some(node) = mat_node {
        if let Some(mut material) = material_without_glow_res.get_mut(&node.0) {
            material.hover_color = color;
        }
    }
    if let Some(node) = mat_glow_node {
        if let Some(mut material) = material_with_glow_res.get_mut(&node.0) {
            material.hover_color = color;
        }
    }
}
