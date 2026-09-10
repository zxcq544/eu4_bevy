use bevy::{
    audio::Volume,
    ecs::relationship::RelatedSpawnerCommands,
    input_focus::tab_navigation::TabIndex,
    picking::hover::Hovered,
    prelude::*,
    ui::InteractionDisabled,
    ui_widgets::{
        Slider, SliderDragState, SliderRange, SliderThumb, SliderValue, TrackClick, ValueChange,
        observe,
    },
};
use settings::Settings;

use crate::plugins::music_player::music_player::BackgroundMusicPlayer;

#[derive(Component, Clone, Default)]
pub struct OptionsUiAudioTab;

pub fn audio_tab(
    options_middle_block: &mut RelatedSpawnerCommands<'_, ChildOf>,
    settings: ResMut<Settings>,
    // localization_res: &Res<Localization>,
    // fonts: &Res<FontHandles>,
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
                    left_block_top.spawn((
                        Text::new("Music volume"),
                        TextLayout {
                            justify: Justify::Center,
                            ..default()
                        },
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
                .with_children(|right_block_top| {
                    right_block_top.spawn((
                        slider(0.0, 1.0, settings.volume_settings.get_music_volume()),
                        observe(|value_change: On<ValueChange<f32>>,
                     mut widget_states: ResMut<DemoWidgetStates>| {
                        widget_states.slider_value = value_change.value;
                    },
                )));
                });
        });
}

// Slider logic - move somewhere else

/// Marker which identifies sliders with a particular style.
#[derive(Component, Default)]
pub struct DemoSlider;

/// Marker which identifies the slider's thumb element.
#[derive(Component, Default)]
pub struct DemoSliderThumb;

// const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
// const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
// const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);
const SLIDER_TRACK: Color = Color::srgb(0.05, 0.05, 0.05);
const SLIDER_THUMB: Color = Color::srgb(0.35, 0.75, 0.35);
// const ELEMENT_OUTLINE: Color = Color::srgb(0.45, 0.45, 0.45);
// const ELEMENT_FILL: Color = Color::srgb(0.35, 0.75, 0.35);
const ELEMENT_FILL_DISABLED: Color = Color::srgb(0.5019608, 0.5019608, 0.5019608);
/// Create a demo slider
fn slider(min: f32, max: f32, value: f32) -> impl Bundle {
    (
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Stretch,
            justify_items: JustifyItems::Center,
            column_gap: px(4),
            height: px(12),
            width: percent(100),
            ..default()
        },
        Name::new("Slider"),
        Hovered::default(),
        DemoSlider,
        Slider {
            track_click: TrackClick::Snap,
            ..Default::default()
        },
        SliderValue(value),
        SliderRange::new(min, max),
        TabIndex(0),
        Children::spawn((
            // Slider background rail
            Spawn((
                Node {
                    height: px(6),
                    border_radius: BorderRadius::all(px(3)),
                    ..default()
                },
                BackgroundColor(SLIDER_TRACK), // Border color for the slider
            )),
            // Invisible track to allow absolute placement of thumb entity. This is narrower than
            // the actual slider, which allows us to position the thumb entity using simple
            // percentages, without having to measure the actual width of the slider thumb.
            Spawn((
                Node {
                    display: Display::Flex,
                    position_type: PositionType::Absolute,
                    left: px(0),
                    // Track is short by 12px to accommodate the thumb.
                    right: px(12),
                    top: px(0),
                    bottom: px(0),
                    ..default()
                },
                children![(
                    // Thumb
                    DemoSliderThumb,
                    SliderThumb,
                    Node {
                        display: Display::Flex,
                        width: px(12),
                        height: px(12),
                        position_type: PositionType::Absolute,
                        left: percent(0), // This will be updated by the slider's value
                        border_radius: BorderRadius::MAX,
                        ..default()
                    },
                    BackgroundColor(SLIDER_THUMB),
                )],
            )),
        )),
    )
}

/// A struct to hold the state of various widgets shown in the demo.
///
/// While it is possible to use the widget's own state components as the source of truth,
/// in many cases widgets will be used to display dynamic data coming from deeper within the app,
/// using some kind of data-binding. This example shows how to maintain an external source of
/// truth for widget states.
#[derive(Resource)]
pub struct DemoWidgetStates {
    pub slider_value: f32,
    pub slider_click: TrackClick,
}

impl FromWorld for DemoWidgetStates {
    fn from_world(world: &mut World) -> Self {
        // 1. Retrieve the existing Settings resource from the world.
        // If Settings might not be loaded yet, use world.get_resource::<Settings>() instead.
        let settings = world.resource::<Settings>();

        // 2. Extract the volume setting you need.
        // (Assuming volume_settings has a field or method returning a f32, like master_volume)
        let initial_volume = settings.volume_settings.get_music_volume();

        // 3. Construct your resource with the dependency fulfilled
        DemoWidgetStates {
            slider_value: initial_volume,
            slider_click: TrackClick::default(), // active fallback/default
        }
    }
}

/// Update the widget states based on the changing resource.
pub fn update_widget_values(
    mut settings: ResMut<Settings>,
    res: Res<DemoWidgetStates>,
    mut sliders: Query<(Entity, &mut Slider), With<DemoSlider>>,
    mut commands: Commands,
    mut background_music_player_query: Query<&mut AudioSink, With<BackgroundMusicPlayer>>,
) {
    if res.is_changed() {
        for (slider_ent, mut slider) in sliders.iter_mut() {
            commands
                .entity(slider_ent)
                .insert(SliderValue(res.slider_value));
            slider.track_click = res.slider_click;
            if let Ok(mut audio_player) = background_music_player_query.single_mut() {
                let volume = Volume::Linear(res.slider_value);
                settings.volume_settings.set_music_volume(res.slider_value);
                audio_player.set_volume(volume);
                // info!("volume {:?}", volume);
            }
            // info!("slider value {:?}", res.slider_value);
        }
    }
}

/// Update the visuals of the slider based on the slider state.
pub fn update_slider_style(
    sliders: Query<
        (
            Entity,
            &SliderValue,
            &SliderRange,
            &Hovered,
            &SliderDragState,
            Has<InteractionDisabled>,
        ),
        (
            Or<(
                Changed<SliderValue>,
                Changed<SliderRange>,
                Changed<Hovered>,
                Changed<SliderDragState>,
                Added<InteractionDisabled>,
            )>,
            With<DemoSlider>,
        ),
    >,
    children: Query<&Children>,
    mut thumbs: Query<(&mut Node, &mut BackgroundColor, Has<DemoSliderThumb>), Without<DemoSlider>>,
) {
    for (slider_ent, value, range, hovered, drag_state, disabled) in sliders.iter() {
        for child in children.iter_descendants(slider_ent) {
            if let Ok((mut thumb_node, mut thumb_bg, is_thumb)) = thumbs.get_mut(child)
                && is_thumb
            {
                thumb_node.left = percent(range.thumb_position(value.0) * 100.0);
                thumb_bg.0 = thumb_color(disabled, hovered.0 | drag_state.dragging);
            }
        }
    }
}

pub fn update_slider_style2(
    sliders: Query<
        (Entity, &Hovered, &SliderDragState, Has<InteractionDisabled>),
        With<DemoSlider>,
    >,
    children: Query<&Children>,
    mut thumbs: Query<(&mut BackgroundColor, Has<DemoSliderThumb>), Without<DemoSlider>>,
    mut removed_disabled: RemovedComponents<InteractionDisabled>,
) {
    removed_disabled.read().for_each(|entity| {
        if let Ok((slider_ent, hovered, drag_state, disabled)) = sliders.get(entity) {
            for child in children.iter_descendants(slider_ent) {
                if let Ok((mut thumb_bg, is_thumb)) = thumbs.get_mut(child)
                    && is_thumb
                {
                    thumb_bg.0 = thumb_color(disabled, hovered.0 | drag_state.dragging);
                }
            }
        }
    });
}

fn thumb_color(disabled: bool, hovered: bool) -> Color {
    match (disabled, hovered) {
        (true, _) => ELEMENT_FILL_DISABLED,

        (false, true) => SLIDER_THUMB.lighter(0.3),

        _ => SLIDER_THUMB,
    }
}
