use bevy::prelude::*;

#[derive(Resource)]
pub struct CursorHandles {
    pub normal: Handle<Image>,
    pub build_cavalry: Handle<Image>,
}
