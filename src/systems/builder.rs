use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::components::*;
use crate::events::*;

pub fn draw_cursor_and_spawn_block(
    camera_q: Single<(&Camera, &GlobalTransform)>,
    window: Query<&Window>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut ray_cast: MeshRayCast,
    mut spawn_block_events: EventWriter<SpawnBlockEvent>,
    mut gizmos: Gizmos,
    block_q: Query<&GlobalTransform, With<Block>>,
    ground_q: Query<&GlobalTransform, With<Ground>>,
    _meshes: Res<Assets<Mesh>>,
) {
    let Ok(window) = window.single() else {
        return;
    };

    let (camera, camera_transform) = *camera_q;

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    let Some((entity, hit)) = ray_cast
        .cast_ray(
            ray,
            &MeshRayCastSettings {
                visibility: RayCastVisibility::Any,
                ..Default::default()
            },
        )
        .first()
    else {
        return;
    };

    let mut pos = None;

    if let Ok(xf) = block_q.get(*entity) {
        let face_center = xf.translation() + (hit.normal * 0.5);

        let block_rotation = xf.rotation();
        let align_to_normal = Quat::from_rotation_arc(block_rotation * Vec3::Z, hit.normal.normalize());
        let final_rotation = align_to_normal * block_rotation;

        gizmos.rect(
            Transform::from_translation(face_center + hit.normal * 0.01) // lil offset for style
                .with_rotation(final_rotation)
                .to_isometry(),
            Vec2::new(1.0, 1.0),
            Color::WHITE,
        );

        pos = Some(face_center);
    }

    if ground_q.get(*entity).is_ok() {
        let mut face = hit.point;
        face.x = face.x.floor() + 0.5;
        face.z = face.z.floor() + 0.5;
        gizmos.rect(
            Transform::from_translation(face + 0.01) // lil offset for style
                .with_rotation(Quat::from_rotation_arc(Vec3::Z, hit.normal))
                .to_isometry(),
            Vec2::new(1.0, 1.0),
            Color::WHITE,
        );
        pos = Some(face);
    }

    if mouse_buttons.just_pressed(MouseButton::Left) {
        if let Some(pos) = pos {
            spawn_block_events.write(SpawnBlockEvent {
                position: pos + hit.normal * 0.5,
                parent: *entity,
            });
        }
    }
}

pub fn spawn_block_on_event(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut spawn_block_events: EventReader<SpawnBlockEvent>,
    query: Query<(Entity, &mut GlobalTransform), With<Block>>,
) {
    for event in spawn_block_events.read() {
        let length = 1.0;
        let half = length * 0.5;

        let block_mesh = meshes.add(Cuboid::from_length(length));
        let block_material = materials.add(Color::srgb(0.2, 0.1, 0.3));

        if let Ok((parent_entity, parent_transform)) = query.get(event.parent) {
            let inverse = parent_transform.compute_matrix().inverse();
            let local_pos = inverse.transform_point3(event.position);

            commands.entity(parent_entity).with_children(|children| {
                children.spawn((
                    Mesh3d(block_mesh),
                    MeshMaterial3d(block_material),
                    Transform::from_translation(local_pos),
                    GlobalTransform::default(),
                    Collider::cuboid(half, half, half),
                    Block,
                ));
            });
        } else {
            commands.spawn((
                Mesh3d(block_mesh),
                MeshMaterial3d(block_material),
                Transform::from_translation(event.position),
                GlobalTransform::default(),
                RigidBody::Dynamic,
                Collider::cuboid(half, half, half),
                ColliderMassProperties::Density(1.0),
                Block,
            ));
        }
    }
}


