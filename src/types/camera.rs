use bevy::prelude::*;

use crate::consts::*;

// The actual world camera, plus any relevant information
// like its position and sensitivity.
#[derive(Component)]
pub struct FPCameraTag;

#[derive(Bundle)]
pub struct FPCamera {
    pub core: FPCameraTag,
    pub camera: Camera3d,
    pub offset: CameraOffset,
    pub sensitivity: CameraSensitivity,
    pub transform: Transform,
}

impl FPCamera {
    pub fn base() -> FPCamera {
        FPCamera {
            core: FPCameraTag,
            camera: Camera3d::default(),
            offset: CameraOffset::new(0.0, CAM_Y0, 0.0),
            sensitivity: CameraSensitivity::default(),
            transform: Transform::from_xyz(PLR_X0, PLR_Y0, PLR_Z0).looking_at(PLR_Y0 * Vec3::Y, Vec3::Y),
        }
    }
}

// Something to store where the camera actually is.
// NOTE: I might restructure this later
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

// Something to store how much the camera angle should
// change depending on how fast the mouse is moved.
// NOTE: I might restructure this later
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
