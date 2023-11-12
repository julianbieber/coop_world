mod grpc_server;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

fn main() {
    let grpc_handle = grpc_server::start_in_background();
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup_graphics)
        .add_systems(Startup, setup_physics)
        .add_systems(Update, print_ball_altitude)
        .run();
    grpc_handle.join().unwrap();
}
fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-3.0, 3.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(RigidBody::Fixed)
        .insert(
            Collider::from_bevy_mesh(&create_plane_mesh(), &ComputedColliderShape::TriMesh)
                .unwrap(),
        )
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(0.5))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)));
}

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        println!("Ball altitude: {}", transform.translation.y);
    }
}

fn create_plane_mesh() -> Mesh {
    let mut m = Mesh::new(bevy::render::render_resource::PrimitiveTopology::TriangleList);
    let mut indices = Vec::with_capacity(6 * 10 * 10);
    for x in 0..10 * 10 {
        indices.push(x * 4);
        indices.push(x * 4 + 2);
        indices.push(x * 4 + 1);
        indices.push(x * 4 + 2);
        indices.push(x * 4 + 3);
        indices.push(x * 4 + 1);
    }
    m.set_indices(Some(bevy::render::mesh::Indices::U32(indices)));

    let mut p: Vec<f32> = Vec::new();
    for y in 0..10 {
        let y = y as f32;
        for x in 0..10 {
            let x = x as f32;
            p.push()
        }
    }
    m.insert_attribute(Mesh::ATTRIBUTE_POSITION, p);

    m
}
