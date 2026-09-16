use crate::{
    core::states::GameState,
    plugins::{
        choose_country::components::bottom_left_block_entity_for_choose_country::{
            BottomLeftBlockButtonActionsForChooseCountry, BottomLeftBlockButtonForChooseCountry,
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

pub const NORMAL_BUTTON: Color = Color::srgb(1.0, 1.0, 1.0);
pub const HOVERED_BUTTON: Color = Color::srgb(1.15, 1.15, 1.15);
pub const PRESSED_BUTTON: Color = Color::srgb(0.85, 0.85, 0.85);

pub fn bottom_left_block_button_system_for_choose_country(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    sound_effects: Res<ButtonClickSoundEffects>,
    settings: Res<Settings>,
    mut input_focus: ResMut<InputFocus>,
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut BottomLeftBlockButtonForChooseCountry,
            &mut ImageNode,
            &BottomLeftBlockButtonActionsForChooseCountry,
        ),
        Changed<Interaction>,
    >,
) {
    for (entity, interaction, mut button, mut image_node, action) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                input_focus.set(entity, FocusCause::Pressed);
                image_node.color = PRESSED_BUTTON;
                // The accessibility system's only update the button's state when the `Button` component is marked as changed.
                button.set_changed();

                // Sound effect logic
                let required_sound = sound_effects.button_click_general.clone();
                commands.spawn((
                    SoundEffectsPlayer,
                    AudioPlayer::new(required_sound),
                    PlaybackSettings {
                        mode: bevy::audio::PlaybackMode::Despawn,
                        volume: Volume::Linear(settings.volume_settings.get_sfx_volume()),
                        ..default()
                    },
                ));

                // Action logic
                match action {
                    BottomLeftBlockButtonActionsForChooseCountry::Options => {
                        info!("Options button pressed");
                        next_state.set(GameState::Options);
                    }
                    BottomLeftBlockButtonActionsForChooseCountry::Back => {
                        info!("Back button pressed");
                        next_state.set(GameState::MainMenu);
                    }
                }
            }
            Interaction::Hovered => {
                input_focus.set(entity, FocusCause::Pressed);
                image_node.color = HOVERED_BUTTON;
                button.set_changed();
            }
            Interaction::None => {
                input_focus.clear();
                image_node.color = NORMAL_BUTTON;
                button.set_changed();
            }
        }
    }
}
