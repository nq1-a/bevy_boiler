use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::consts::*;
use crate::types::object::*;

#[derive(Component)]
pub struct PlayerTag;

#[derive(Bundle)]
pub struct Player {
    pub core: PlayerTag,
    pub phys: PhysicsObject,
    pub skin: ContactSkin,
    pub impulse: ExternalImpulse,
    pub damping: Damping,
    pub locked_axes: LockedAxes,
    pub jump_cooldown: JumpCooldown,
}

impl Player {
    pub fn base() -> Player {
        Player {
            core: PlayerTag,
            phys: PhysicsObject::base(
                RigidBody::Dynamic,
                Collider::capsule_y(PLR_H4, PLR_H4),
                Transform::from_xyz(PLR_X0, PLR_Y0, PLR_Z0).looking_at(PLR_Y0 * Vec3::Y, Vec3::Y),
            ),
            skin: ContactSkin(PLR_CE * PLR_JE),
            impulse: ExternalImpulse::default(),
            damping: Damping {
                linear_damping: 1.5,
                angular_damping: 1.0,
            },
            // X and Z axes (only yaw)
            locked_axes: LockedAxes::from_bits(40u8).unwrap(),
            jump_cooldown: JumpCooldown::new(PLR_JC),
        }
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
