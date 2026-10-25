use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        //.add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, scene.spawn())
        .run();
}

fn scene() -> impl SceneList {
    bsn_list! {
        #Player
        Camera3d
        Transform::from_xyz(-7.0, 7.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y)
        --
        Collider::cuboid(100.0, 1.0, 100.0)
        Transform::from_xyz(0.0, 0.0, 0.0)
        --
        RigidBody::Dynamic
        Collider::ball(0.5)
        Restitution::coefficient(0.7)
        Mesh3d(asset_value(Sphere::new(0.5)))
        MeshMaterial3d::<StandardMaterial>(asset_value(Color::WHITE))
        Transform::from_xyz(0.0, 4.0, 0.0)
    }
}
