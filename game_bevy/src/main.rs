use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy 2D - Debug Mode".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    cameras: Query<Entity, With<Camera2d>>,
) {
    // Kill default 3D camera
    for camera in cameras.iter() {
        commands.entity(camera).despawn();
    }

    // Spawn a 2D camera
    commands.spawn((Camera2d::default(), Transform::from_xyz(0.0, 0.0, 10.0)));

    // Spawn a bright neon green rectangle (definitely visible)
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(200.0, 100.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.0, 1.0, 0.0))), // Neon green
        Transform::from_xyz(-250.0, 0.0, 0.0),
    ));

    // Load sprite from assets
    let image_handle =
        asset_server.load("kenney_playing-cards-pack/PNG/Cards (large)/card_back.png");

    // Spawn sprite to the right with different size
    commands.spawn((
        Sprite {
            image: image_handle,
            custom_size: Some(Vec2::new(1000.0, 1500.0)),
            color: Color::srgb(1.0, 1.0, 1.0),
            ..default()
        },
        Transform::from_xyz(250.0, 0.0, 0.0),
    ));

    println!("Spawned: green rectangle (left) + sprite (right)");
}
