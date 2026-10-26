use std::f32::consts::FRAC_PI_2;

use bevy::{
    input::mouse::AccumulatedMouseMotion,
    window::{CursorGrabMode, CursorOptions},
    prelude::*,
};
use bevy_rapier3d::prelude::*;

mod types;
use types::*;

static X0: f32 = 10.0;
static Y0: f32 = 5.0;
static Z0: f32 = 0.0;
static RX_MAX: f32 = FRAC_PI_2 - 0.01;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
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
    let ball = meshes.add(Sphere::new(0.5));
    let mat1 = materials.add(Color::WHITE);

    // Add player
    commands.spawn((
        Player,
        RigidBody::Dynamic,
        Collider::capsule_y(0.25, 0.25),
        Transform::from_xyz(X0, Y0, Z0).looking_at(Y0 * Vec3::Y, Vec3::Y),
    ));

    // Add camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(X0, Y0, Z0).looking_at(Y0 * Vec3::Y, Vec3::Y),
    ));

    // Add ground
    commands.spawn((
        Collider::cuboid(100.0, 2.0, 100.0),
        Transform::from_xyz(0.0, -1.0, 0.0),
    ));

    // Add objects
    commands.spawn((
        RigidBody::Dynamic,
        Collider::ball(0.5),
        Restitution::coefficient(1.0),
        Mesh3d(ball.clone()),
        MeshMaterial3d(mat1.clone()),
        Transform::from_xyz(0.0, 4.0, 0.0),
    ));

    commands.spawn((
        RigidBody::Fixed,
        Collider::ball(0.5),
        Mesh3d(ball.clone()),
        MeshMaterial3d(mat1.clone()),
        Transform::from_xyz(0.0, 10.0, 0.0),
    ));
}

fn move_player(
    time: Res<Time<Fixed>>,
    kb_input: Res<ButtonInput<KeyCode>>,
    mouse_dp: Res<AccumulatedMouseMotion>,
    plr: Single<&mut Transform, With<Player>>,
    cam: Single<&mut Transform, (With<Camera3d>, Without<Player>)>,
) {
    let mut plr_t = plr.into_inner();
    let mut cam_t = cam.into_inner();

    // Rotation
    let delta = mouse_dp.delta;
    let delta_ry = -delta.x * 0.0025;
    let delta_rx = -delta.y * 0.0025;
    let (mut ry, mut rx, _) = cam_t.rotation.to_euler(EulerRot::YXZ);

    if delta != Vec2::ZERO {
        ry =  ry + delta_ry;
        rx = (rx + delta_rx).clamp(-RX_MAX, RX_MAX);
    }

    plr_t.rotation = Quat::from_euler(EulerRot::YXZ, ry, 0.0, 0.0);
    cam_t.rotation = Quat::from_euler(EulerRot::YXZ, ry,  rx, 0.0);

    let ru = plr_t.rotation * Vec3::NEG_Z;
    let su = ru.rotate_y(FRAC_PI_2);

    // Calculate translation
    let mut rm: f32 = 0.0;
    let mut sm: f32 = 0.0;

    if kb_input.pressed(KeyCode::KeyW) {rm += 1.0;}
    if kb_input.pressed(KeyCode::KeyS) {rm -= 1.0;}
    if kb_input.pressed(KeyCode::KeyA) {sm += 1.0;}
    if kb_input.pressed(KeyCode::KeyD) {sm -= 1.0;}

    let dv: Vec3 = (rm * ru + sm * su).normalize_or_zero();
    plr_t.translation += time.delta_secs() * dv;

    // Update translation
    cam_t.translation = plr_t.translation;
}
