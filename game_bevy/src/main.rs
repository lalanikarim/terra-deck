use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy 2D - Large Sprite".to_string(),
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

    // Spawn a large colored square
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(400.0, 400.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.2, 0.8, 0.2))),
        Transform::from_xyz(-250.0, 0.0, 0.0),
    ));

    // Load sprite - 64x64 pixels
    let card_back = asset_server.load("kenney_playing-cards-pack/PNG/Cards (large)/card_back.png");

    // Spawn sprite with very large custom size to make small sprite visible
    commands.spawn((
        Sprite {
            image: card_back,
            custom_size: Some(Vec2::new(1200.0, 1200.0)), // Make tiny 64x64 sprite huge
            color: Color::srgb(1.0, 1.0, 1.0),
            ..default()
        },
        Transform::from_xyz(350.0, 0.0, 0.0),
    ));

    println!("Spawned: colored square (left) + ENLARGED sprite (right - 1200×1200)");
}
