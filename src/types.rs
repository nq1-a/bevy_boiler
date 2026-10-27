use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::consts::*;

#[derive(Component)]
pub struct Player;

type BasePlayer = (
    Player,
    RigidBody,
    Collider,
    Transform,
    ExternalImpulse,
    Velocity,
    Damping,
);

impl Player {
    pub fn base() -> BasePlayer {
        (
            Self,
            RigidBody::Dynamic,
            Collider::capsule_y(PLR_H4, PLR_H4),
            Transform::from_xyz(PLR_X0, PLR_Y0, PLR_Z0).looking_at(PLR_Y0 * Vec3::Y, Vec3::Y),
            ExternalImpulse::default(),
            Velocity::default(),
            Damping {
                linear_damping: 1.2,
                angular_damping: 1.0,
            },
        )
    }
}

#[derive(Component)]
pub struct FPCamera;

type BaseFPCamera = (
    FPCamera,
    Camera3d,
    CameraOffset,
    CameraSensitivity,
    Transform,
);

impl FPCamera {
    pub fn base() -> BaseFPCamera {
        (
            Self,
            Camera3d::default(),
            CameraOffset::new(0.0, CAM_Y0, 0.0),
            CameraSensitivity::default(),
            Transform::from_xyz(PLR_X0, PLR_Y0, PLR_Z0).looking_at(PLR_Y0 * Vec3::Y, Vec3::Y),
        )
    }
}

#[derive(Component)]
pub struct CameraOffset(pub Vec3);

impl CameraOffset {
    pub fn new(x: f32, y: f32, z: f32) -> Self
        {Self(Vec3::new(x, y, z))}
}

impl Default for CameraOffset {
    fn default() -> Self
        {Self(Vec3::ZERO.clone())}
}

#[derive(Component)]
pub struct CameraSensitivity(pub Vec2);

impl CameraSensitivity {
    pub fn new(x: f32, y: f32) -> Self
        {Self(Vec2::new(x, y))}
}

impl Default for CameraSensitivity {
    fn default() -> Self
        {Self(Vec2::new(CAM_SX, CAM_SY))}
}
