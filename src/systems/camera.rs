use bevy::prelude::*;

pub fn move_cam(
    input: Res<ButtonInput<KeyCode>>,
    mut camera_q: Query<(&mut Transform, &mut Projection), With<Camera>>,
    time: Res<Time>,
) {
    let Ok((mut transform, mut projection)) = camera_q.single_mut() else {
        return;
    };

    let camera_speed = 30.0;
    let height_offset = 2.0;
    let min_scale = 0.5;
    let max_scale = 4.0;
    let scale_change_speed = 0.02;

    let mut axis = Vec3::ZERO;
    for key in input.get_pressed() {
        axis += match key {
            KeyCode::KeyA => transform.left().as_vec3(),
            KeyCode::KeyD => transform.right().as_vec3(),
            KeyCode::KeyW => transform.up().as_vec3(),
            KeyCode::KeyS => transform.down().as_vec3(),
            _ => Vec3::ZERO,
        };
    }

    if let Projection::Orthographic(ortho) = &mut *projection {
        if input.pressed(KeyCode::KeyE) {
            ortho.scale = (ortho.scale - scale_change_speed).max(min_scale);
        }
        if input.pressed(KeyCode::KeyQ) {
            ortho.scale = (ortho.scale + scale_change_speed).min(max_scale);
        }
    }

    let mut new_translation =
        transform.translation + axis.normalize_or_zero() * (camera_speed * time.delta_secs());

    new_translation.y = new_translation.y.max(height_offset);

    transform.translation = new_translation;
    transform.look_at(Vec3::Y * height_offset, Vec3::Y);
}


