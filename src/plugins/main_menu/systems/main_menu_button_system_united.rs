use crate::{
    core::states::GameState,
    plugins::{
        main_menu::{
            components::main_menu_entity::{
                MainMenuButton, MainMenuButtonAction, MainMenuButtonSoundType,
            },
            resources::exit_delay_timer::ExitDelayTimer,
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
pub const HOVERED_BUTTON: Color = Color::srgb(1.25, 1.25, 1.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.75, 0.75, 0.75);

pub fn main_menu_button_system_united(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    mut input_focus: ResMut<InputFocus>,
    mut exit_timer: ResMut<ExitDelayTimer>,
    settings: Res<Settings>,
    button_click_sound_effects: Res<ButtonClickSoundEffects>,
    // Get all components in one query
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut MainMenuButton,
            &mut ImageNode,
            &MainMenuButtonSoundType,
            &MainMenuButtonAction,
        ),
        Changed<Interaction>,
    >,
) {
    for (entity, interaction, mut button, mut image_node, sound_type, action) in
        &mut interaction_query
    {
        match *interaction {
            Interaction::Pressed => {
                // 1. Color logic
                input_focus.set(entity, FocusCause::Pressed);
                image_node.color = PRESSED_BUTTON;
                button.set_changed();

                // 2. Sound effect logic
                let required_sound = match sound_type {
                    MainMenuButtonSoundType::Ok => {
                        button_click_sound_effects.button_click_ok.clone()
                    }
                    MainMenuButtonSoundType::Back => {
                        button_click_sound_effects.button_click_back.clone()
                    }
                    MainMenuButtonSoundType::General => {
                        button_click_sound_effects.button_click_general.clone()
                    }
                };
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

                // 3. Action logic
                match action {
                    MainMenuButtonAction::Options => {
                        info!("Options button pressed");
                        next_state.set(GameState::Options);
                    }
                    MainMenuButtonAction::Quit => {
                        info!("Quit button pressed");
                        let exit_time_setting = settings.exit_delay_time;
                        exit_timer.timer = Timer::from_seconds(exit_time_setting, TimerMode::Once);
                        exit_timer.should_exit = true;
                    }
                    _ => {}
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
