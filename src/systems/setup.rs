use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_rapier3d::prelude::*;
use crate::components::*;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor_size = Vec3::new(10.0, 0.1, 10.0);

    let floor = (
        Mesh3d(meshes.add(Cuboid::from_size(floor_size))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::from_translation(Vec3::ZERO),
        GlobalTransform::default(),
        Collider::cuboid(floor_size.x * 0.5, floor_size.y * 0.5, floor_size.z * 0.5),
        RigidBody::Fixed,
        Ground,
    );

    commands.spawn(floor);

    // light
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..DirectionalLight::default()
        },
        Transform::from_translation(Vec3::new(2.5, 10.0, 5.0)).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // camera
    commands.spawn((
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: (8.0),
            },
            scale: 1.0,

            ..OrthographicProjection::default_3d()
        }),
        Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}


