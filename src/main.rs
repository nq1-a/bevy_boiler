use bevy::{
    input::mouse::AccumulatedMouseMotion,
    window::{CursorGrabMode, CursorOptions, PresentMode, PrimaryWindow},
    prelude::*,
};
use bevy_rapier3d::prelude::*;

mod consts;
mod game;
mod types;

use consts::*;
use game::*;
use types::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        //.add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, (
            lock_cursor,
            setup_ui,
            setup,
        ))
        .add_systems(Update, (
            fade_logo.run_if(|time: Res<Time>| {
                time.elapsed_secs() < GUI_LM
            }),
            move_player,
        ))
        .run();
}

// TODO: Make cursor locking toggleable for menus
fn lock_cursor(
    p_window: Single<(&mut CursorOptions, &mut Window), With<PrimaryWindow>>,
) {
    let (mut cursor, mut window) = p_window.into_inner();
    window.present_mode = PresentMode::Mailbox;
    cursor.grab_mode = CursorGrabMode::Locked;
    cursor.visible = false;
}

fn setup_ui(
    mut commands: Commands,
    assets: ResMut<AssetServer>,
) {
    commands.spawn((
        Node {
            width: percent(100.0),
            height: percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
            (
                SplashUi,
                ImageNode::new(assets.load("game/logo.png")),
                Node {
                    border: px(0.0).all(),
                    padding: px(0.0).all(),
                    width: vmin(50.0),
                    height: vmin(50.0),
                    ..default()
                },
            ),
        )],
    ));
}

fn fade_logo(
    time: Res<Time>,
    logo: Single<&mut ImageNode, With<SplashUi>>,
) {
    let mut img = logo.into_inner();
    img.color.set_alpha(
        (1.0 - (time.elapsed_secs() - GUI_L1) / GUI_L2)
            .clamp(0.0, 1.0)
            .powi(5)
    );
}

fn move_player(
    phys_ctx: ReadRapierContext,
    time: Res<Time>,
    kb_input: Res<ButtonInput<KeyCode>>,
    mouse_dp: Res<AccumulatedMouseMotion>,
    plr: Single<(Entity, &mut JumpCooldown, &mut Transform, &mut ExternalImpulse), With<PlayerTag>>,
    cam: Single<(&mut Transform, &CameraOffset, &CameraSensitivity), (With<FPCameraTag>, Without<PlayerTag>)>,
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
        rx = (rx + delta_rx).clamp(-GLB_RX, GLB_RX);
    }

    plr_t.rotation = Quat::from_euler(EulerRot::YXZ, ry, 0.0, 0.0);
    cam_t.rotation = Quat::from_euler(EulerRot::YXZ, ry,  rx, 0.0);

    let ru = plr_t.rotation * Vec3::NEG_Z;
    let su = ru.rotate_y(FRAC_PI_2);

    // Calculate new translation
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
