use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::random;

use crate::consts::*;

// Network data bundles everything that might need to be
// shared via a host-peer connection.
#[derive(Component)]
pub struct NetworkData {
    // The unique ID of the object
    pub id: u64,
    // The unique ID of the owner
    pub owner: u64,
}

impl NetworkData {
    pub fn new() -> Self {
        Self {
            id: random(),
            owner: *NET_ID,
        }
    }
}

// A physics object can represent any object that lives in
// the physics simulation and interacts with other objects.
// NOTE: I might move fixed bodies to a different object
// depending on how I feel in the future.
#[derive(Bundle)]
pub struct PhysicsObject {
    pub body: RigidBody,
    pub mesh: Mesh3d,
    pub collider: Collider,
    pub transform: Transform,
    pub velocity: Velocity,
    pub friction: Friction,
    pub netdata: NetworkData,
}

impl PhysicsObject {
    pub fn base(
        body: RigidBody,
        mesh: &MeshCarrier,
        transform: Transform,
    ) -> Self {
        Self {
            body,
            mesh: Mesh3d(mesh.handle.clone()),
            collider: mesh.collider.clone(),
            transform,
            velocity: Velocity::default(),
            friction: Friction {
                coefficient: GLB_FD,
                ..default()
            },
            netdata: NetworkData::new(),
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
    ) -> Self {
        Self {
            handle: server.add(mesh.into()),
            collider: Collider::from_bevy_mesh(
                &mesh.into(),
                &ComputedColliderShape::ConvexHull
            ).unwrap(),
        }
    }
}
