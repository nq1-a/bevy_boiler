use bevy::{
    input::mouse::AccumulatedMouseMotion,
    window::{CursorGrabMode, CursorOptions},
    prelude::*,
};
use bevy_rapier3d::prelude::*;

mod consts;
mod types;
use consts::*;
use types::*;

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
        Player::base(),
    ));

    // Add camera
    commands.spawn((
        FPCamera::base(),
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
    phys_ctx: ReadRapierContext,
    time: Res<Time<Fixed>>,
    kb_input: Res<ButtonInput<KeyCode>>,
    mouse_dp: Res<AccumulatedMouseMotion>,
    plr: Single<(Entity, &mut JumpCooldown, &mut Transform, &mut ExternalImpulse), With<Player>>,
    cam: Single<(&mut Transform, &CameraOffset, &CameraSensitivity), (With<Camera3d>, Without<Player>)>,
) {
    let phys_ctx = phys_ctx.single().unwrap();
    let (plr_ent, mut plr_jc, mut plr_t, mut plr_imp) = plr.into_inner();
    let (mut cam_t, cam_o, cam_s) = cam.into_inner();

    // Rotation
    let delta = mouse_dp.delta;
    let delta_ry = -delta.x * cam_s.0.x;
    let delta_rx = -delta.y * cam_s.0.y;
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
    let dt = time.delta_secs();
    let mut rm: f32 = 0.0;
    let mut sm: f32 = 0.0;

    if kb_input.pressed(KeyCode::KeyW) {rm += PLR_DV;}
    if kb_input.pressed(KeyCode::KeyS) {rm -= PLR_DV;}
    if kb_input.pressed(KeyCode::KeyA) {sm += PLR_DV;}
    if kb_input.pressed(KeyCode::KeyD) {sm -= PLR_DV;}

    let mut dv: Vec3 = (rm * ru + sm * su).normalize_or_zero();
    if kb_input.pressed(KeyCode::ShiftLeft) {dv *= PLR_SM;}
    plr_t.translation += dt * dv;

    // Update translation
    plr_jc.update(dt);
    println!("{}", plr_jc.remaining());
    if  kb_input.pressed(KeyCode::Space) &&
        plr_jc.remaining() == 0.0 &&
        phys_ctx.cast_ray(
            plr_t.translation,
            Vec3::NEG_Y,
            PLR_H2 + PLR_JE,
            true,
            QueryFilter::default().exclude_rigid_body(plr_ent),
        ).is_some()
    {
        plr_imp.impulse = Vec3::new(0.0, PLR_DH, 0.0);
        plr_jc.reset();
    }

    cam_t.translation = plr_t.translation + cam_o.0;
}
