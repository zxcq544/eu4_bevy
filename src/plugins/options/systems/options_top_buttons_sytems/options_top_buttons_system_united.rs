use crate::{
    core::states::OptionsTabState,
    plugins::sound_effects::resources::button_click_sound_effects::ButtonClickSoundEffects,
};
use bevy::{
    audio::Volume,
    input_focus::{FocusCause, InputFocus},
    prelude::*,
};
use settings::Settings;

#[derive(Component, Clone, Default, Reflect)]
pub struct OptionsTopTabButton;

#[derive(Component, Clone, Default)]
pub enum OptionsTopTabButtonAction {
    #[default]
    NoAction,
    Video,
    Audio,
    Game,
    Controls,
}

pub const NORMAL_BUTTON: Color = Color::srgb(1.0, 1.0, 1.0);
pub const HOVERED_BUTTON: Color = Color::srgb(1.15, 1.15, 1.15);
pub const PRESSED_BUTTON: Color = Color::srgb(0.85, 0.85, 0.85);

pub fn options_top_buttons_system_united(
    mut commands: Commands,
    mut next_state: ResMut<NextState<OptionsTabState>>,
    sound_effects: Res<ButtonClickSoundEffects>,
    settings: Res<Settings>,
    mut input_focus: ResMut<InputFocus>,
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut OptionsTopTabButton,
            // &mut ImageNode,
            &mut Node,
            &OptionsTopTabButtonAction,
        ),
        Changed<Interaction>,
    >,
) {
    for (entity, interaction, mut button, mut node, action) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                input_focus.set(entity, FocusCause::Pressed);
                node.border = UiRect::all(Val::Px(20.0));
                // image_node.color = PRESSED_BUTTON;
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
                    OptionsTopTabButtonAction::Video => {
                        info!("Video button pressed");
                        next_state.set(OptionsTabState::Video);
                        // settings.initial_bootscreen_show_time = 2.0;
                        // save_settings_to_json(&settings);
                    }
                    OptionsTopTabButtonAction::Audio => {
                        info!("Audio button pressed");
                        next_state.set(OptionsTabState::Audio);
                        // settings.initial_bootscreen_show_time = 2.0;
                        // save_settings_to_json(&settings);
                    }
                    OptionsTopTabButtonAction::Game => {
                        info!("Game button pressed");
                        next_state.set(OptionsTabState::Game);
                        // settings.initial_bootscreen_show_time = 2.0;
                        // save_settings_to_json(&settings);
                    }
                    OptionsTopTabButtonAction::Controls => {
                        info!("Controls button pressed");
                        next_state.set(OptionsTabState::Controls);
                        // settings.initial_bootscreen_show_time = 2.0;
                        // save_settings_to_json(&settings);
                    }
                    OptionsTopTabButtonAction::NoAction => {
                        // info!("No action button pressed");
                    }
                }
            }
            Interaction::Hovered => {
                input_focus.set(entity, FocusCause::Pressed);
                node.border = UiRect::all(Val::Px(10.0));
                // image_node.color = HOVERED_BUTTON;
                button.set_changed();
            }
            Interaction::None => {
                input_focus.clear();
                node.border = UiRect::all(Val::Px(0.0));
                // image_node.color = NORMAL_BUTTON;
                button.set_changed();
            }
        }
    }
}

// fn save_settings_to_json(settings: &Settings) {
//     let file_path = Path::new(&settings.file_name);
//     info!("Saving settings to {:?}", file_path);
//     if file_path.exists() {
//         let mut file = File::create(file_path)
//             .expect("file settings.json doesn't exist in root of game or blocked for write");
//         serde_json::to_writer_pretty(&mut file, &settings).expect("Unable to write to file");
//     } else {
//         info!("File settings.json doesn't exist in root of game or blocked for write");
//     }
// }
