use bevy::prelude::*;

mod component;
mod system;
mod input;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(system::setup)
        .add_system(input::ship_movement)
        .add_system(input::ship_rotation)
        .add_system(system::box_movement)
        .run();
}
