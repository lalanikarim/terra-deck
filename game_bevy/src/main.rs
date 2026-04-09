use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy 2D - Card Sprite Demo".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut cameras: Query<&mut Transform, With<Camera2d>>,
) {
    // Position the 2D camera at Z=10 (in front) to view sprites at Z=0
    for mut camera_transform in cameras.iter_mut() {
        camera_transform.translation.x = 0.0;
        camera_transform.translation.y = 0.0;
        camera_transform.translation.z = 10.0;
    }

    // Load card sprite from local assets folder
    let image_handle =
        asset_server.load("kenney_playing-cards-pack/PNG/Cards (medium)/card_back.png");

    // Spawn sprite with custom size
    commands.spawn((
        Sprite {
            image: image_handle,
            custom_size: Some(Vec2::new(300.0, 450.0)),
            ..default()
        },
        Transform::default(),
    ));

    println!("✅ Sprite loaded successfully!");
}
