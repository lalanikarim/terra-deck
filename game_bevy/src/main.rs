use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy 2D Scene".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    cameras: Query<Entity, With<Camera2d>>,
) {
    // Kill default 3D camera and spawn a 2D camera instead
    for camera in cameras.iter() {
        commands.entity(camera).despawn();
    }

    // Spawn a 2D camera
    commands.spawn(Camera2d::default());

    // Solid green rectangle in 2D - using Mesh2d + MeshMaterial2d
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(20.0, 12.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.2, 0.8, 0.2))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}
