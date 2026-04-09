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
    cameras: Query<Entity, With<Camera2d>>,
) {
    // Kill default 3D camera and spawn a 2D camera instead
    for camera in cameras.iter() {
        commands.entity(camera).despawn();
    }

    // Spawn a 2D camera
    commands.spawn(Camera2d::default());

    // Try Sprite approach with load() - returns Image handle
    let card_back: Handle<Image> =
        asset_server.load("kenney_playing-cards-pack/PNG/Cards (medium)/card_back.png");

    // Spawn using Sprite with the loaded image
    commands.spawn(Sprite {
        image: card_back,
        custom_size: Some(Vec2::new(1.0, 1.5)),
        ..default()
    });

    println!("Sprite loaded successfully!");
}
