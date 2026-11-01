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
    LockedAxes,
    JumpCooldown,
);

impl Player {
    pub fn base() -> BasePlayer {
        (
            Self,
            RigidBody::Dynamic,
            Collider::capsule_y(PLR_H4, PLR_H4),
            ContactSkin(PLR_CE * PLR_JE),
            Transform::from_xyz(PLR_X0, PLR_Y0, PLR_Z0).looking_at(PLR_Y0 * Vec3::Y, Vec3::Y),
            ExternalImpulse::default(),
            Velocity::default(),
            Damping {
                linear_damping: 1.5,
                angular_damping: 1.0,
            },
            Friction {
                coefficient: GLB_FD,
                ..default()
            },
            // X and Z axes (only yaw)
            LockedAxes::from_bits(40u8).unwrap(),
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
