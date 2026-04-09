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
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut cameras: Query<&mut Transform, With<Camera2d>>,
) {
    // Position the 2D camera at Z=10 (in front) to view sprites at Z=0
    for mut camera_transform in cameras.iter_mut() {
        camera_transform.translation.x = 0.0;
        camera_transform.translation.y = 0.0;
        camera_transform.translation.z = 10.0; // Z=10 to look at Z=0 sprites
    }

    // Load the card sprite from project assets folder
    let card_back: Handle<Image> =
        asset_server.load("../assets/kenney_playing-cards-pack/PNG/Cards (medium)/card_back.png");

    // Spawn sprite with custom size - set image and custom_size
    commands.spawn((
        Sprite {
            image: card_back,
            custom_size: Some(Vec2::new(300.0, 450.0)),
            ..default()
        },
        Transform::default(),
    ));

    println!("Sprite loaded successfully!");
}
