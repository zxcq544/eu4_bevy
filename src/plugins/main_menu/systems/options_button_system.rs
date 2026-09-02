use crate::{
    core::states::MainMenuStates,
    plugins::{
        main_menu::components::{
            continue_game_entity::ContinueGameEntity,
            main_menu_entity::MainMenuEntity,
            options_entity::{OptionsButton, OptionsButtonAction},
        },
        sound_effects::resources::button_click_sound_effects::ButtonClickSoundEffects,
    },
};
use bevy::{
    audio::Volume,
    input_focus::{FocusCause, InputFocus},
    prelude::*,
};
use settings::Settings;

pub const NORMAL_BUTTON: Color = Color::srgb(1.0, 1.0, 1.0);
pub const HOVERED_BUTTON: Color = Color::srgb(1.15, 1.15, 1.15);
pub const PRESSED_BUTTON: Color = Color::srgb(0.85, 0.85, 0.85);

pub fn options_button_system(
    mut commands: Commands,
    mut next_state: ResMut<NextState<MainMenuStates>>,
    sound_effects: Res<ButtonClickSoundEffects>,
    settings: Res<Settings>,
    mut input_focus: ResMut<InputFocus>,
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut OptionsButton,
            &mut ImageNode,
            &OptionsButtonAction,
        ),
        Changed<Interaction>,
    >,
    mut query_to_unhide_main_menu_and_continue_blocks: Query<
        &mut Visibility,
        Or<(With<MainMenuEntity>, With<ContinueGameEntity>)>,
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
                    AudioPlayer::new(required_sound),
                    PlaybackSettings {
                        mode: bevy::audio::PlaybackMode::Despawn,
                        volume: Volume::Linear(settings.volume_settings.sound_effects_volume),
                        speed: 1.0,
                        paused: false,
                        muted: false,
                        ..default()
                    },
                ));

                // Action logic
                match action {
                    OptionsButtonAction::Apply => {
                        info!("Apply button pressed");
                    }
                    OptionsButtonAction::Back => {
                        info!("Back button pressed");
                        // unhide main menu block and continue game block
                        for mut visibility in &mut query_to_unhide_main_menu_and_continue_blocks {
                            *visibility = Visibility::Visible;
                        }
                        next_state.set(MainMenuStates::OptionsHidden);
                    }
                    OptionsButtonAction::NoAction => {
                        // info!("No action button pressed");
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
