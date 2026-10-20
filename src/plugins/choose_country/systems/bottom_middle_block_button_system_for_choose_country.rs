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

pub fn bottom_middle_block_button_system_for_choose_country(
    mut commands: Commands,
    // mut next_state: ResMut<NextState<GameState>>,
    sound_effects: Res<ButtonClickSoundEffects>,
    settings: Res<Settings>,
    mut input_focus: ResMut<InputFocus>,
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut BottomMiddleBlockButtonForChooseCountry,
            Option<&MaterialNode<CustomUiMaterial>>,
            Option<&MaterialNode<CustomUiMaterialWithGlow>>,
        ),
        Changed<Interaction>,
    >,
    mut material_without_glow_res: ResMut<Assets<CustomUiMaterial>>,
    mut material_with_glow_res: ResMut<Assets<CustomUiMaterialWithGlow>>,
) {
    for (entity, interaction, mut button, material_without_glow_node, material_with_glow_node) in
        &mut interaction_query
    {
        // 1. Get target color
        let target_color = match *interaction {
            Interaction::Pressed => PRESSED_BUTTON,
            Interaction::Hovered => HOVERED_BUTTON,
            Interaction::None => NORMAL_BUTTON,
        };

        // 2. Change material color of button
        if let Some(node) = material_without_glow_node {
            if let Some(mut material) = material_without_glow_res.get_mut(&node.0) {
                material.hover_color = target_color;
            }
        }
        if let Some(node) = material_with_glow_node {
            if let Some(mut material) = material_with_glow_res.get_mut(&node.0) {
                material.hover_color = target_color;
            }
        }

        // 3. Action logic
        match *interaction {
            Interaction::Pressed => {
                input_focus.set(entity, FocusCause::Pressed);
                button.set_changed();

                // Sound logic
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

                info!("Bottom middle block button pressed");
                // next_state.set(GameState::MainMenu);
            }
            Interaction::Hovered => {
                input_focus.set(entity, FocusCause::Pressed);
                button.set_changed();
            }
            Interaction::None => {
                input_focus.clear();
                button.set_changed();
            }
        }
    }
}
