use crate::components::Block;
use bevy::prelude::*;

const DESPAWN_HEIGHT: f32 = -100.0;

pub fn check_and_despawn_block(
    mut commands: Commands,
    block_query: Query<(Entity, &GlobalTransform), With<Block>>
) {
    for (entity, global_transform) in block_query.iter() {
        if global_transform.translation().y < DESPAWN_HEIGHT {
            commands.entity(entity).despawn();
        }
    }
}
