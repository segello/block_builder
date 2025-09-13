use bevy::prelude::*;
use block_builder::RootPlugin;

fn main() {
    App::new()
        .add_plugins(RootPlugin)
        .run();
}

