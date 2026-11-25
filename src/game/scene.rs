use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::consts::*;
use crate::types::*;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Load meshes & materials
    let capsule = MeshCarrier::new(&mut meshes, Capsule3d::new(PLR_H4, PLR_H2));
    let ball = MeshCarrier::new(&mut meshes, Sphere::new(0.5));
    let mat1 = materials.add(Color::WHITE);

    // Add player & camera
    commands.spawn(Player::base(&capsule));
    commands.spawn(FPCamera::base());

    // Add light
    commands.spawn((
        DirectionalLight {
            color: Color::srgb(0.6, 0.63, 0.7),
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::new(0.3, -1.0, 0.3), Vec3::Y),
    ));

    // Add ground
    commands.spawn((
        Collider::cuboid(100.0, 2.0, 100.0),
        Transform::from_xyz(0.0, -1.0, 0.0),
        Friction {
            coefficient: GLB_FD,
            ..default()
        },
    ));

    // Add objects
    commands.spawn((
        PhysicsObject::base(
            RigidBody::Fixed,
            &ball,
            Transform::from_xyz(0.0, 10.0, 0.0),
        ),
        MeshMaterial3d(mat1.clone()),
    ));

    commands.spawn((
        PhysicsObject::base(
            RigidBody::Dynamic,
            &ball,
            Transform::from_xyz(0.0, 4.0, 0.0),
        ),
        Restitution::coefficient(1.0),
        MeshMaterial3d(mat1.clone()),
    ));
}
