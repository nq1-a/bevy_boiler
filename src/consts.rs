pub use std::f32::consts::FRAC_PI_2;

// RX_MAX - Maximum camera pitch
pub static RX_MAX: f32 = FRAC_PI_2 - 0.01;

// CAM_SX, CAM_SY - Sensitivy of camera on both axes
// CAM_Y0 - Starting camera offset from player center
pub static CAM_SX: f32 = 0.0025;
pub static CAM_SY: f32 = 0.0025;
pub static CAM_Y0: f32 = 0.375;

// PLR_DH - Jump height scalar
// PLR_DV - Walking speed scalar
// PLR_EJ - Maximum error bound for v_y ~ 0
// PLR_H, PLR_H4 - Player height and quarter-height
// PLR_SM - Sprint speed scalar
// PLR_X0, PLR_Y0, PLR_Z0 - Player starting position
pub static PLR_DH: f32 = 0.75;
pub static PLR_DV: f32 = 2.0;
pub static PLR_EJ: f32 = 0.001;
pub static PLR_H:  f32 = 1.0;
pub static PLR_H4: f32 = 0.25 * PLR_H;
pub static PLR_SM: f32 = 1.5;
pub static PLR_X0: f32 = 10.0;
pub static PLR_Y0: f32 = 5.0;
pub static PLR_Z0: f32 = 0.0;
