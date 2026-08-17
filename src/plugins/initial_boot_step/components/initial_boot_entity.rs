use bevy::prelude::*;

#[derive(Component, Default, Clone)]
pub struct InitialBootEntity;

impl InitialBootEntity {
    pub fn as_scene(background_image: Handle<Image>) -> impl Scene {
        bsn! {
            InitialBootEntity
            Node{
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                overflow: Overflow::hidden()
            }
            Children[
                Node{
                    width: Val::Percent(100.0),
                    height: Val::Auto,
                }
                ImageNode{
                    image: background_image,
                    image_mode: NodeImageMode::Auto,
                }
            ]
        }
    }
}
