use bevy::prelude::*;

use crate::consts::*;

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
