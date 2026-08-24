use bevy::prelude::*;

#[derive(Resource)]
pub struct BackgroundImageForContinue {
    pub image: Handle<Image>,
}