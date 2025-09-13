use bevy::prelude::*;

#[derive(Event)]
pub struct SpawnBlockEvent {
    pub position: Vec3,
    pub parent: Entity,
}


