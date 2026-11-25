use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::consts::*;

// A physics object can represent any object that lives in
// the physics simulation and interacts with other objects.
// NOTE: I might move fixed bodies to a different object
// depending on how I feel in the future.
#[derive(Component)]
pub struct PhysicsObjectTag;

#[derive(Bundle)]
pub struct PhysicsObject {
    pub core: PhysicsObjectTag,
    pub body: RigidBody,
    pub mesh: Mesh3d,
    pub collider: Collider,
    pub transform: Transform,
    pub velocity: Velocity,
    pub friction: Friction,
}

impl PhysicsObject {
    pub fn base(
        body: RigidBody,
        mesh: &MeshCarrier,
        transform: Transform,
    ) -> PhysicsObject {
        PhysicsObject {
            core: PhysicsObjectTag,
            body,
            mesh: Mesh3d(mesh.handle.clone()),
            collider: mesh.collider.clone(),
            transform,
            velocity: Velocity::default(),
            friction: Friction {
                coefficient: GLB_FD,
                ..default()
            },
        }
    }
}

// Contains both a mesh and its computed collider so we
// only have to calculate it once.
pub struct MeshCarrier {
    pub handle: Handle<Mesh>,
    pub collider: Collider,
}

impl MeshCarrier {
    pub fn new(
        server: &mut ResMut<Assets<Mesh>>,
        mesh: impl Into<Mesh> + Copy,
    ) -> MeshCarrier {
        MeshCarrier {
            handle: server.add(mesh.into()),
            collider: Collider::from_bevy_mesh(
                &mesh.into(),
                &ComputedColliderShape::ConvexHull
            ).unwrap(),
        }
    }
}
