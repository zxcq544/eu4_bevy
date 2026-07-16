use bevy::prelude::*;
mod systems;
use systems::{hello_bevy, hello_world};

fn main() {
    App::new()
        .add_systems(Startup, hello_world)
        .add_systems(Startup, hello_bevy)
        .run();
}
