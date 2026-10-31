use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::consts::*;

#[derive(Component)]
pub struct Player;

type BasePlayer = (
    Player,
    RigidBody,
    Collider,
    ContactSkin,
    Transform,
    ExternalImpulse,
    Velocity,
    Damping,
    Friction,
    JumpCooldown,
);

impl Player {
    pub fn base() -> BasePlayer {
        (
            Self,
            RigidBody::Dynamic,
            Collider::capsule_y(PLR_H4, PLR_H4),
            ContactSkin(PLR_JE),
            Transform::from_xyz(PLR_X0, PLR_Y0, PLR_Z0).looking_at(PLR_Y0 * Vec3::Y, Vec3::Y),
            ExternalImpulse::default(),
            Velocity::default(),
            Damping {
                linear_damping: 1.5,
                angular_damping: 1.0,
            },
            Friction {
                coefficient: 0.7,
                combine_rule: CoefficientCombineRule::Average,
            },
            JumpCooldown::new(PLR_JC),
        )
    }
}

#[derive(Component)]
pub struct JumpCooldown(f32, f32);

impl JumpCooldown {
    pub fn new(length: f32) -> Self
        {Self(length, length)}

    pub fn remaining(&self) -> f32
        {self.1}

    pub fn update(&mut self, dt: f32) {
        self.1 = (self.1 - dt).max(0.0);
    }

    pub fn reset(&mut self) {
        self.1 = self.0;
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
        {Self(Vec3::ZERO)}
}

#[derive(Component)]
pub struct CameraSensitivity(pub Vec2);

impl CameraSensitivity {
    pub fn new(x: f32, y: f32) -> Self
        {Self(Vec2::new(x, y))}
}

impl Default for CameraSensitivity {
    fn default() -> Self
        {Self::new(CAM_SX, CAM_SY)}
}
