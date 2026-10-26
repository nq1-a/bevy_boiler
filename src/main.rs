use std::f32::consts::FRAC_PI_2;

use bevy::{
    input::mouse::AccumulatedMouseMotion,
    window::{CursorGrabMode, CursorOptions},
    prelude::*,
};
use bevy_rapier3d::prelude::*;

mod types;
use types::*;

static RX_MAX: f32 = FRAC_PI_2 - 0.01;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        //.add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, lock_cursor)
        .add_systems(Startup, setup)
        .add_systems(Update, move_player)
        .run();
}

fn lock_cursor(
    mut window: Query<&mut CursorOptions, With<Window>>,
) {
    let mut cursor = window.single_mut().unwrap();
    cursor.grab_mode = CursorGrabMode::Locked;
    cursor.visible = false;
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Load meshes & materials
    let ball1 = meshes.add(Sphere::new(0.5));
    let mat1 = materials.add(Color::WHITE);

    // Add objects
    commands.spawn((
        Player,
        RigidBody::Dynamic,
        Collider::ball(0.5),
        Transform::from_xyz(10.0, 0.5, 0.0).looking_at(0.5 * Vec3::Y, Vec3::Y),
        Camera3d::default(),
    ));

    commands.spawn((
        Collider::cuboid(100.0, 1.0, 100.0),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    commands.spawn((
        RigidBody::Dynamic,
        Collider::ball(0.5),
        Restitution::coefficient(0.7),
        Mesh3d(ball1.clone()),
        MeshMaterial3d(mat1.clone()),
        Transform::from_xyz(0.0, 4.0, 0.0),
    ));

    commands.spawn((
        RigidBody::Fixed,
        Collider::ball(0.5),
        Mesh3d(ball1.clone()),
        MeshMaterial3d(mat1.clone()),
        Transform::from_xyz(0.0, 10.0, 0.0),
    ));
}

fn move_player(
    mouse_dp: Res<AccumulatedMouseMotion>,
    player: Single<&mut Transform, With<Player>>,
) {
    let mut transform = player.into_inner();
    let delta = mouse_dp.delta;

    if delta != Vec2::ZERO {
        let delta_ry = -delta.x * 0.003;
        let delta_rx = -delta.y * 0.002;

        let (ry, rx, rz) = transform.rotation.to_euler(EulerRot::YXZ);
        let ry =  ry + delta_ry;
        let rx = (rx + delta_rx).clamp(-RX_MAX, RX_MAX);

        transform.rotation = Quat::from_euler(EulerRot::YXZ, ry, rx, rz);
    }
}
