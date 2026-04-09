use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy 2D - Card Assets".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(game_bevy::plugins::CardAssetPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    cameras: Query<Entity, With<Camera2d>>,
) {
    println!("🎴 Card assets being loaded via CardAssetPlugin...");

    // Kill default 3D camera
    for camera in cameras.iter() {
        commands.entity(camera).despawn();
    }

    // Spawn a 2D camera
    commands.spawn((Camera2d::default(), Transform::from_xyz(0.0, 0.0, 10.0)));

    // Load card back for initial display
    let card_back = asset_server.load("kenney_playing-cards-pack/PNG/Cards (large)/card_back.png");

    // Spawn card in the center with proper ratio
    commands.spawn((
        Sprite {
            image: card_back,
            custom_size: Some(Vec2::new(300.0, 450.0)),
            color: Color::srgb(1.0, 1.0, 1.0),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    println!("✅ Main card spawned");
}
