use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::consts::*;

#[derive(Component)]
pub struct PhysicsObjectTag;

#[derive(Bundle)]
pub struct PhysicsObject {
    pub core: PhysicsObjectTag,
    pub body: RigidBody,
    pub collider: Collider,
    pub transform: Transform,
    pub velocity: Velocity,
    pub friction: Friction,
}

impl PhysicsObject {
    pub fn base(
        body: RigidBody,
        collider: Collider,
        transform: Transform,
    ) -> PhysicsObject {
        PhysicsObject {
            core: PhysicsObjectTag,
            body,
            collider,
            transform,
            velocity: Velocity::default(),
            friction: Friction {
                coefficient: GLB_FD,
                ..default()
            },
        }
    }
}
