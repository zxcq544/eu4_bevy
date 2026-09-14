use bevy::audio::Volume;
use bevy::prelude::*;
use bevy::ui_widgets::{SetSliderValue, SliderValueChange};
use bevy::{
    ecs::{
        entity::Entity,
        system::{Commands, Query, Res, ResMut},
    },
    input_focus::{FocusCause, InputFocus},
    ui::Interaction,
};
use settings::Settings;

use crate::plugins::options::components::options_middle_block::audio_slider_widget::slider_left_arrow_button::OptionsUIAudioSliderButtonLeft;
use crate::plugins::sound_effects::components::sound_effects_player::SoundEffectsPlayer;
use crate::plugins::sound_effects::resources::button_click_sound_effects::ButtonClickSoundEffects;

pub const NORMAL_BUTTON: Color = Color::srgb(1.0, 1.0, 1.0);
pub const HOVERED_BUTTON: Color = Color::srgb(1.15, 1.15, 1.15);
pub const PRESSED_BUTTON: Color = Color::srgb(0.85, 0.85, 0.85);

pub fn volume_decrease_system(
    mut commands: Commands,
    settings: Res<Settings>,
    sound_effects: Res<ButtonClickSoundEffects>,
    mut input_focus: ResMut<InputFocus>,
    mut interaction_query: Query<
        (
            Entity,
            &mut OptionsUIAudioSliderButtonLeft,
            &Interaction,
            &mut ImageNode,
        ),
        Changed<Interaction>,
    >,
) {
    for (entity, mut button, interaction, mut image_node) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                input_focus.set(entity, FocusCause::Pressed);
                // node.border = UiRect::all(Val::Px(20.0));
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
                        speed: 1.0,
                        paused: false,
                        muted: false,
                        ..default()
                    },
                ));
                commands.trigger(SetSliderValue {
                    change: SliderValueChange::Absolute(
                        settings.volume_settings.get_sfx_volume() - 0.1,
                    ),
                    entity: entity,
                });
                info!("Volume decreased");
            }
            Interaction::Hovered => {
                input_focus.set(entity, FocusCause::Pressed);
                // node.border = UiRect::all(Val::Px(10.0));
                image_node.color = HOVERED_BUTTON;
                button.set_changed();
            }
            Interaction::None => {
                input_focus.clear();
                // node.border = UiRect::all(Val::Px(0.0));
                image_node.color = NORMAL_BUTTON;
                button.set_changed();
            }
        }
    }
}
