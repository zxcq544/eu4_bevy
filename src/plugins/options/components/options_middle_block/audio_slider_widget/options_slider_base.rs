use crate::plugins::game_main::resources::game_ui_resources::GameUiResources;
use bevy::{
    input_focus::tab_navigation::TabIndex,
    picking::hover::Hovered,
    prelude::*,
    ui::InteractionDisabled,
    ui_widgets::{Slider, SliderDragState, SliderRange, SliderThumb, SliderValue, TrackClick},
};

/// Marker which identifies sliders with a particular style.
#[derive(Component, Default)]
pub struct OptionUiAudioSlider;

/// Marker which identifies the slider's thumb element.
#[derive(Component, Default)]
pub struct OptionsUiAudioSliderThumb;

// const SLIDER_TRACK: Color = Color::srgb(0.05, 0.05, 0.05);
const SLIDER_THUMB: Color = Color::srgb(1.0, 1.0, 1.0);
const ELEMENT_FILL_DISABLED: Color = Color::srgb(0.5019608, 0.5019608, 0.5019608);

pub fn slider(min: f32, max: f32, value: f32, ui_resources: &Res<GameUiResources>) -> impl Bundle {
    (
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Stretch,
            justify_items: JustifyItems::Center,
            column_gap: px(4),
            height: px(16),
            width: percent(100),
            ..default()
        },
        Name::new("Slider"),
        Hovered::default(),
        OptionUiAudioSlider,
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
                ImageNode {
                    image: ui_resources.ui_slider_track_image.clone(),
                    image_mode: NodeImageMode::Sliced(TextureSlicer {
                        border: BorderRect::all(6.0),
                        center_scale_mode: SliceScaleMode::Stretch,
                        sides_scale_mode: SliceScaleMode::Stretch,
                        max_corner_scale: 1.0,
                        ..default()
                    }),
                    ..default()
                },
                // BackgroundColor(SLIDER_TRACK), // Border color for the slider
            )),
            // Invisible track to allow absolute placement of thumb entity. This is narrower than
            // the actual slider, which allows us to position the thumb entity using simple
            // percentages, without having to measure the actual width of the slider thumb.
            Spawn((
                Node {
                    display: Display::Flex,
                    position_type: PositionType::Absolute,
                    left: px(0),
                    // Track is short by 16px to accommodate the thumb.
                    right: px(16),
                    top: px(0),
                    bottom: px(0),
                    ..default()
                },
                children![(
                    // Thumb
                    OptionsUiAudioSliderThumb,
                    SliderThumb,
                    Node {
                        display: Display::Flex,
                        width: px(16),
                        height: px(16),
                        position_type: PositionType::Absolute,
                        left: percent(0), // This will be updated by the slider's value
                        border_radius: BorderRadius::MAX,
                        ..default()
                    },
                    ImageNode {
                        image: ui_resources.ui_slider_thumb_image_blue.clone(),
                        image_mode: NodeImageMode::Stretch,
                        color: SLIDER_THUMB,
                        ..default()
                    },
                )],
            )),
        )),
    )
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
            With<OptionUiAudioSlider>,
        ),
    >,
    children: Query<&Children>,
    mut thumbs: Query<
        (&mut Node, &mut ImageNode, Has<OptionsUiAudioSliderThumb>),
        Without<OptionUiAudioSlider>,
    >,
) {
    for (slider_ent, value, range, hovered, drag_state, disabled) in sliders.iter() {
        for child in children.iter_descendants(slider_ent) {
            if let Ok((mut thumb_node, mut thumb_image_node, is_thumb)) = thumbs.get_mut(child)
                && is_thumb
            {
                thumb_node.left = percent(range.thumb_position(value.0) * 100.0);
                thumb_image_node.color = thumb_color(disabled, hovered.0 | drag_state.dragging);
            }
        }
    }
}

// Used to set disable i think
// pub fn update_slider_style2(
//     sliders: Query<
//         (Entity, &Hovered, &SliderDragState, Has<InteractionDisabled>),
//         With<OptionUiAudioSlider>,
//     >,
//     children: Query<&Children>,
//     mut thumbs: Query<
//         (&mut BackgroundColor, Has<OptionsUiAudioSliderThumb>),
//         Without<OptionUiAudioSlider>,
//     >,
//     mut removed_disabled: RemovedComponents<InteractionDisabled>,
// ) {
//     removed_disabled.read().for_each(|entity| {
//         if let Ok((slider_ent, hovered, drag_state, disabled)) = sliders.get(entity) {
//             for child in children.iter_descendants(slider_ent) {
//                 if let Ok((mut thumb_bg, is_thumb)) = thumbs.get_mut(child)
//                     && is_thumb
//                 {
//                     thumb_bg.0 = thumb_color(disabled, hovered.0 | drag_state.dragging);
//                 }
//             }
//         }
//     });
// }

fn thumb_color(disabled: bool, hovered: bool) -> Color {
    match (disabled, hovered) {
        (true, _) => ELEMENT_FILL_DISABLED,

        (false, true) => Color::srgb(1.25, 1.25, 1.25),

        _ => SLIDER_THUMB,
    }
}
