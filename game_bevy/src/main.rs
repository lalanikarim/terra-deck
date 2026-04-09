use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy 2D - Card Ratio".to_string(),
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
    cameras: Query<Entity, With<Camera2d>>,
) {
    // Kill default 3D camera
    for camera in cameras.iter() {
        commands.entity(camera).despawn();
    }

    // Spawn a 2D camera
    commands.spawn((Camera2d::default(), Transform::from_xyz(0.0, 0.0, 10.0)));

    // Load sprite - 64x64 but should be 2:3 ratio (64x96)
    let card_back = asset_server.load("kenney_playing-cards-pack/PNG/Cards (large)/card_back.png");

    // Spawn sprite with proper card aspect ratio
    commands.spawn((
        Sprite {
            image: card_back,
            // Card ratio is typically 2:3 or 1:1.5
            custom_size: Some(Vec2::new(600.0, 900.0)),
            color: Color::srgb(1.0, 1.0, 1.0),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    println!("Spawned enlarged card sprite (600×900) centered");
}
