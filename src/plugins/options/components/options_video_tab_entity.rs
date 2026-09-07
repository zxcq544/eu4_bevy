use bevy::{
    feathers::{
        controls::{
            FeathersMenu, FeathersMenuButton, FeathersMenuDivider, FeathersMenuItem,
            FeathersMenuPopup,
        },
        theme::ThemedText,
    },
    prelude::*,
    ui_widgets::Activate,
};

#[derive(Component, Clone, Default)]
pub struct OptionsVideoTabEntity;

impl OptionsVideoTabEntity {
    pub fn as_scene_list() -> impl SceneList {
        bsn_list! {
            OptionsVideoTabEntity
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexStart,
                // background_color: Color::TRANSPARENT,
                // overflow: Overflow::hidden(),
            }
            Visibility::Hidden // Spawn as hidden because we don't despawn ingame ui elements
            ZIndex(3)
            BackgroundColor(Color::NONE)
            Children [
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(49.0),
                    height: Val::Vh(40.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    top: Val::Percent(35.0),
                    // bottom: Val::Px(3.0),
                }
                Children [
                    // Video tab
                    Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        // bottom: Val::Px(3.0),
                    }
                    Outline {
                        color: Color::srgb_from_array([0.2, 0.8, 0.8]),
                        width: Val::Px(2.0),
                    }

                    @FeathersMenu
                    Children [
                        (
                            @FeathersMenuButton {
                                @caption: bsn! { Text("Menu") ThemedText },
                            }
                            AccessibleLabel("Menu Example")
                            Node {
                                flex_grow: 1.0,
                            }
                        ),
                        (
                            @FeathersMenuPopup
                            Children [
                                (
                                    @FeathersMenuItem {
                                        @caption: bsn! { Text("MenuItem 1") ThemedText },
                                    }
                                    on(|_: On<Activate>| {
                                        info!("Menu item 1 clicked!");
                                    })
                                ),
                                (
                                    @FeathersMenuItem {
                                        @caption: bsn! { Text("MenuItem 2") ThemedText },
                                    }
                                    on(|_: On<Activate>| {
                                        info!("Menu item 2 clicked!");
                                    })
                                ),
                                @FeathersMenuDivider,
                                (
                                    @FeathersMenuItem {
                                        @caption: bsn! { Text("MenuItem 3") ThemedText },
                                    }
                                    on(|_: On<Activate>| {
                                        info!("Menu item 3 clicked!");
                                    })
                                ),
                            ]
                        ),
                    ]
                ]
            ]
        }
    }
}
