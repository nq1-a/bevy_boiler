pub use std::f32::consts::FRAC_PI_2;

// GLB_FD - Default friction
// GLB_RX - Maximum controller pitch
pub static GLB_FD: f32 = 0.4;
pub static GLB_RX: f32 = FRAC_PI_2 - 0.01;

// CAM_SX, CAM_SY - Sensitivy of camera on both axes
// CAM_Y0 - Starting camera offset from player center
pub static CAM_SX: f32 = 0.0025;
pub static CAM_SY: f32 = 0.0025;
pub static CAM_Y0: f32 = 0.3 * PLR_H;

// GUI_L1 - How long the logo lasts before fading out
// GUI_L2 - How long it takes for the logo to fade out
// GUI_LM - Total time spent dealing with logo screen
pub static GUI_L1: f32 = 2.0;
pub static GUI_L2: f32 = 0.5;
pub static GUI_LM: f32 = GUI_L1 + GUI_L2 + 0.1;

// PLR_CE - Contact skin thickness / PLR_JE
// PLR_DH - Jump height scalar
// PLR_DV - Walking speed scalar
// PLR_H, PLR_H2, PLR_H4 - Player height portions
// PLR_JC - Cooldown between jumps
// PLR_JE - Gap for raycast with ground during jump
// PLR_SM - Sprint speed scalar
// PLR_X0, PLR_Y0, PLR_Z0 - Player starting position
pub static PLR_CE: f32 = 0.4;
pub static PLR_DH: f32 = 0.5;
pub static PLR_DV: f32 = 3.0;
pub static PLR_H:  f32 = 1.0;
pub static PLR_H2: f32 = 0.5 * PLR_H;
pub static PLR_H4: f32 = 0.25 * PLR_H;
pub static PLR_JC: f32 = 0.7;
pub static PLR_JE: f32 = 0.002;
pub static PLR_SM: f32 = 2.0;
pub static PLR_X0: f32 = 10.0;
pub static PLR_Y0: f32 = 5.0;
pub static PLR_Z0: f32 = 0.0;
