mod components;
mod events;
mod systems;

use bevy::prelude::*;

use events::*;
use systems::*;

pub struct BuilderPlugin;

impl Plugin for BuilderPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SpawnBlockEvent>()
            .add_systems(Startup, setup)
            .add_systems(Update, draw_cursor_and_spawn_block)
            .add_systems(Update, spawn_block_on_event)
            .add_systems(Update, move_cam)
            .add_systems(Update, quit_on_escape)
            .add_systems(Update, check_and_despawn_block);
    } 
}

pub struct RootPlugin;

impl Plugin for RootPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(DefaultPlugins)
            .add_plugins(bevy_rapier3d::prelude::RapierPhysicsPlugin::<
                bevy_rapier3d::plugin::NoUserData,
            >::default())
            .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
            .add_plugins(bevy::diagnostic::LogDiagnosticsPlugin::default())
            .add_plugins(BuilderPlugin);
    }
}
